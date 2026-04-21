use crate::domain::entity::token::ApiToken;
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn list(&self, page: u32, page_size: u32) -> DomainResult<Vec<ApiToken>>;
    async fn create(
        &self,
        name: &str,
        scopes: Vec<String>,
        expiration_mode: Option<&str>,
        custom_expires_at: Option<&str>,
    ) -> DomainResult<ApiToken>;
    async fn delete(&self, id: &str) -> DomainResult<()>;
    async fn update_expiration(
        &self,
        id: &str,
        expires_at: Option<&str>,
    ) -> DomainResult<()>;
}
