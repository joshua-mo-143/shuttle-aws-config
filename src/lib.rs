use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use shuttle_service::Factory;
use shuttle_service::ResourceBuilder;
use shuttle_service::Type;
use aws_config::Region;
pub use aws_config::SdkConfig;
use aws_credential_types::Credentials;

#[derive(Serialize, Deserialize)]
pub struct ClientOutput {
pub    access_key_id: String,
pub    secret_access_key: String,
pub    session_token: Option<String>,
pub    region: String
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Client {
pub    access_key_id: String,
pub    secret_access_key: String,
pub    session_token: Option<String>,
pub    region: String
}

impl Client {
    pub fn access_key_id(mut self, access_key_id: &str) -> Self {
        self.access_key_id = access_key_id.to_string();

        self
    }
    pub fn secret_access_key(mut self, secret_access_key: &str) -> Self {
        self.secret_access_key = secret_access_key.to_string();

        self
    }
    pub fn session_token(mut self, session_token: &str) -> Self {
        self.session_token = Some(session_token.to_string());

        self
    }
    pub fn region(mut self, region: &str) -> Self {
        self.region = region.to_string();

        self
    }
}

#[async_trait]
impl ResourceBuilder<SdkConfig> for Client {
    const TYPE: Type = Type::Custom;

    type Config = Self;

    type Output = ClientOutput;

    fn new() -> Self {
        Self {
            access_key_id: String::new(),
            secret_access_key: String::new(),
            session_token: None,
            region: String::new(),
        }
    }

    fn config(&self) -> &Self::Config {
        self
    }

    async fn output(
        self,
        _factory: &mut dyn Factory,
    ) -> Result<Self::Output, shuttle_service::Error> {
        Ok(
            ClientOutput {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
            session_token: self.session_token,
            region: self.region,
                }
            )
    }

    async fn build(build_data: &Self::Output) -> Result<SdkConfig, shuttle_service::Error> {
        let creds = Credentials::from_keys(
               build_data.access_key_id.clone(),
               build_data.secret_access_key.clone(),
               build_data.session_token.clone() 
            ); 
        
        let cfg = aws_config::from_env()
            .region(Region::new(build_data.region.clone()))
            .credentials_provider(creds)
            .load()
            .await;

        Ok(cfg)
    }
}
