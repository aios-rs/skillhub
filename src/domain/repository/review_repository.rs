use crate::domain::entity::review::ReviewTask;
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn list(&self, status: Option<&str>) -> DomainResult<Vec<ReviewTask>>;
    async fn approve(&self, id: &str, comment: Option<&str>) -> DomainResult<()>;
    async fn reject(&self, id: &str, comment: Option<&str>) -> DomainResult<()>;
}
