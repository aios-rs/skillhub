use crate::domain::entity::user::{ProfileChangeRequest, UserProfile};
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_profile(&self) -> DomainResult<Option<UserProfile>>;
    async fn update_profile(
        &self,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> DomainResult<UserProfile>;
    async fn list_profile_change_requests(&self) -> DomainResult<Vec<ProfileChangeRequest>>;
    async fn approve_profile_change(&self, id: &str, reason: Option<&str>) -> DomainResult<()>;
    async fn reject_profile_change(&self, id: &str, reason: Option<&str>) -> DomainResult<()>;
    async fn cancel_profile_change(&self, id: &str) -> DomainResult<()>;
}
