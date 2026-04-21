use crate::domain::entity::namespace::{Namespace, NamespaceMember};
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait NamespaceRepository: Send + Sync {
    async fn list(&self) -> DomainResult<Vec<Namespace>>;
    async fn create(
        &self,
        slug: &str,
        display_name: &str,
        namespace_type: &str,
        description: Option<&str>,
    ) -> DomainResult<Namespace>;
    async fn update(
        &self,
        id: &str,
        display_name: Option<&str>,
        description: Option<&str>,
    ) -> DomainResult<Namespace>;
    async fn delete(&self, id: &str) -> DomainResult<()>;
    async fn list_members(&self, namespace_id: &str) -> DomainResult<Vec<NamespaceMember>>;
    async fn add_member(
        &self,
        namespace_id: &str,
        user_id: &str,
        role: &str,
    ) -> DomainResult<NamespaceMember>;
    async fn remove_member(&self, namespace_id: &str, user_id: &str) -> DomainResult<()>;
    async fn update_member_role(
        &self,
        namespace_id: &str,
        user_id: &str,
        role: &str,
    ) -> DomainResult<()>;
    async fn list_tenant_users(&self) -> DomainResult<Vec<(String, String)>>;
}
