use crate::domain::entity::report::SkillReport;
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn submit(
        &self,
        namespace: &str,
        slug: &str,
        reason: &str,
        description: Option<&str>,
    ) -> DomainResult<SkillReport>;
    async fn list(&self) -> DomainResult<Vec<SkillReport>>;
    async fn resolve(&self, id: &str, resolution: &str, action_taken: Option<&str>)
        -> DomainResult<()>;
    async fn dismiss(&self, id: &str) -> DomainResult<()>;
    async fn hide_skill(&self, id: &str, reason: Option<&str>) -> DomainResult<()>;
    async fn unhide_skill(&self, id: &str) -> DomainResult<()>;
}
