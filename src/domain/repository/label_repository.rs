use crate::domain::entity::label::{Label, LabelTranslation};
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait LabelRepository: Send + Sync {
    async fn list(&self) -> DomainResult<Vec<Label>>;
    async fn create(
        &self,
        slug: &str,
        label_type: &str,
        translations: Vec<LabelTranslation>,
        visible_in_filter: bool,
        sort_order: i32,
    ) -> DomainResult<Label>;
    async fn update(
        &self,
        id: &str,
        translations: Vec<LabelTranslation>,
        visible_in_filter: Option<bool>,
        sort_order: Option<i32>,
    ) -> DomainResult<Label>;
    async fn delete(&self, id: &str) -> DomainResult<()>;
}
