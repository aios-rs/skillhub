use crate::domain::entity::skill::{
    HubStats, PublishResult, Skill, SkillFile, SkillLifecycleMutationResponse, SkillVersion,
};
use crate::domain::error::DomainResult;
use crate::domain::value_object::sort_order::SortOrder;
use crate::domain::value_object::visibility::Visibility;
use async_trait::async_trait;

#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub query: Option<String>,
    pub namespace: Option<String>,
    pub labels: Vec<String>,
    pub visibility: Option<String>,
    pub sort: SortOrder,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub skills: Vec<Skill>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
}

#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn search(&self, params: SearchParams) -> DomainResult<SearchResult>;
    async fn get_detail(&self, namespace: &str, slug: &str) -> DomainResult<Option<Skill>>;
    async fn list_versions(
        &self,
        namespace: &str,
        slug: &str,
        page: u32,
        page_size: u32,
    ) -> DomainResult<Vec<SkillVersion>>;
    async fn get_version(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Option<SkillVersion>>;
    async fn list_files(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<SkillFile>>;
    async fn get_file_content(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
        path: &str,
    ) -> DomainResult<String>;
    async fn publish(
        &self,
        namespace: &str,
        file_data: Vec<u8>,
        visibility: Visibility,
    ) -> DomainResult<PublishResult>;
    async fn download_bundle(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<u8>>;
    async fn download_latest(&self, namespace: &str, slug: &str) -> DomainResult<Vec<u8>>;
    async fn star(&self, skill_id: &str) -> DomainResult<()>;
    async fn unstar(&self, skill_id: &str) -> DomainResult<()>;
    async fn rate(&self, skill_id: &str, score: i16) -> DomainResult<()>;
    async fn get_stats(&self) -> DomainResult<HubStats>;
    async fn list_my_skills(&self, page: u32, page_size: u32) -> DomainResult<SearchResult>;
    async fn list_my_stars(&self, page: u32, page_size: u32) -> DomainResult<SearchResult>;
    async fn archive(&self, namespace: &str, slug: &str) -> DomainResult<SkillLifecycleMutationResponse>;
    async fn unarchive(
        &self,
        namespace: &str,
        slug: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse>;
    async fn yank_version(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
        reason: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse>;
    async fn rerelease_version(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse>;
    async fn delete_version(&self, namespace: &str, slug: &str, version: &str) -> DomainResult<()>;
    async fn submit_review(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
        comment: Option<&str>,
    ) -> DomainResult<SkillLifecycleMutationResponse>;
    async fn withdraw_review(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse>;
    async fn confirm_publish(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<SkillLifecycleMutationResponse>;
    async fn get_skill_labels(&self, skill_id: &str) -> DomainResult<Vec<String>>;
    async fn set_skill_labels(&self, skill_id: &str, label_ids: Vec<i64>) -> DomainResult<()>;
    async fn remove_label_from_skill(&self, skill_id: &str, label_id: i64) -> DomainResult<()>;
}
