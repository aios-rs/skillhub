use crate::domain::entity::notification::{Notification, NotificationPreference};
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn list(
        &self,
        notification_type: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> DomainResult<Vec<Notification>>;
    async fn get_unread_count(&self) -> DomainResult<i64>;
    async fn mark_as_read(&self, id: &str) -> DomainResult<()>;
    async fn mark_all_as_read(&self) -> DomainResult<()>;
    async fn delete(&self, id: &str) -> DomainResult<()>;
    async fn get_preferences(&self) -> DomainResult<Vec<NotificationPreference>>;
    async fn update_preference(
        &self,
        notification_type: &str,
        enabled: bool,
    ) -> DomainResult<()>;
}
