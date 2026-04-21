use async_trait::async_trait;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn login(&self, username: &str, password: &str) -> Result<String, crate::domain::error::DomainError>;
    async fn login_with_app(&self, app_id: &str, app_secret: &str) -> Result<String, crate::domain::error::DomainError>;
}
