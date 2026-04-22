use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn login(&self, username: &str, password: &str) -> Result<AuthTokens, crate::domain::error::DomainError>;
    async fn login_with_app(&self, app_id: &str, app_secret: &str) -> Result<AuthTokens, crate::domain::error::DomainError>;
}
