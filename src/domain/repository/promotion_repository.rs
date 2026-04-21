use crate::domain::entity::promotion::Promotion;
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait PromotionRepository: Send + Sync {
    async fn submit(
        &self,
        source_skill_id: i64,
        source_version_id: i64,
        target_namespace_id: i64,
    ) -> DomainResult<Promotion>;
    async fn list(&self, status: Option<&str>, page: u32, page_size: u32)
        -> DomainResult<Vec<Promotion>>;
    async fn get_detail(&self, id: &str) -> DomainResult<Option<Promotion>>;
    async fn approve(&self, id: &str, comment: Option<&str>) -> DomainResult<()>;
    async fn reject(&self, id: &str, comment: Option<&str>) -> DomainResult<()>;
}
