use std::sync::Arc;
use crate::domain::{
    repository::{
        AuthRepository, LabelRepository, NamespaceRepository, NotificationRepository,
        PromotionRepository, ReportRepository, ReviewRepository,
        SkillRepository, TokenRepository, UserRepository,
    },
    error::{DomainError, DomainResult},
    entity::{
        skill::{HubStats, Skill, SkillFile, SkillVersion},
        namespace::Namespace,
        label::Label,
        notification::{Notification, NotificationPreference},
        review::ReviewTask,
        token::ApiToken,
        user::UserProfile,
    },
};

pub struct SkillHubService {
    auth_repo: Option<Arc<dyn AuthRepository>>,
    skill_repo: Arc<dyn SkillRepository>,
    namespace_repo: Option<Arc<dyn NamespaceRepository>>,
    label_repo: Option<Arc<dyn LabelRepository>>,
    review_repo: Option<Arc<dyn ReviewRepository>>,
    report_repo: Option<Arc<dyn ReportRepository>>,
    promotion_repo: Option<Arc<dyn PromotionRepository>>,
    notification_repo: Option<Arc<dyn NotificationRepository>>,
    token_repo: Option<Arc<dyn TokenRepository>>,
    user_repo: Option<Arc<dyn UserRepository>>,
}

impl SkillHubService {
    pub fn new(skill_repo: Arc<dyn SkillRepository>) -> Self {
        Self {
            auth_repo: None,
            skill_repo,
            namespace_repo: None,
            label_repo: None,
            review_repo: None,
            report_repo: None,
            promotion_repo: None,
            notification_repo: None,
            token_repo: None,
            user_repo: None,
        }
    }

    pub fn with_auth_repo(mut self, repo: Arc<dyn AuthRepository>) -> Self {
        self.auth_repo = Some(repo);
        self
    }

    pub fn with_namespace_repo(mut self, repo: Arc<dyn NamespaceRepository>) -> Self {
        self.namespace_repo = Some(repo);
        self
    }

    pub fn with_label_repo(mut self, repo: Arc<dyn LabelRepository>) -> Self {
        self.label_repo = Some(repo);
        self
    }

    pub fn with_review_repo(mut self, repo: Arc<dyn ReviewRepository>) -> Self {
        self.review_repo = Some(repo);
        self
    }

    pub fn with_report_repo(mut self, repo: Arc<dyn ReportRepository>) -> Self {
        self.report_repo = Some(repo);
        self
    }

    pub fn with_promotion_repo(mut self, repo: Arc<dyn PromotionRepository>) -> Self {
        self.promotion_repo = Some(repo);
        self
    }

    pub fn with_notification_repo(mut self, repo: Arc<dyn NotificationRepository>) -> Self {
        self.notification_repo = Some(repo);
        self
    }

    pub fn with_token_repo(mut self, repo: Arc<dyn TokenRepository>) -> Self {
        self.token_repo = Some(repo);
        self
    }

    pub fn with_user_repo(mut self, repo: Arc<dyn UserRepository>) -> Self {
        self.user_repo = Some(repo);
        self
    }

    // Skill methods
    pub async fn search_skills(
        &self,
        query: Option<String>,
        namespace: Option<String>,
        labels: Vec<String>,
        sort: String,
        page: u32,
        page_size: u32,
    ) -> DomainResult<(Vec<Skill>, i64)> {
        let params = crate::domain::repository::skill_repository::SearchParams {
            query,
            namespace,
            labels,
            visibility: None,
            sort: sort.try_into().map_err(|e: String| DomainError::InvalidInput(e))?,
            page,
            page_size,
        };
        let result = self.skill_repo.search(params).await?;
        Ok((result.skills, result.total))
    }

    pub async fn get_skill_detail(&self, namespace: &str, slug: &str) -> DomainResult<Option<Skill>> {
        self.skill_repo.get_detail(namespace, slug).await
    }

    pub async fn list_versions(
        &self,
        namespace: &str,
        slug: &str,
        page: u32,
        page_size: u32,
    ) -> DomainResult<Vec<SkillVersion>> {
        self.skill_repo.list_versions(namespace, slug, page, page_size).await
    }

    pub async fn get_version(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Option<SkillVersion>> {
        self.skill_repo.get_version(namespace, slug, version).await
    }

    pub async fn list_files(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<SkillFile>> {
        self.skill_repo.list_files(namespace, slug, version).await
    }

    pub async fn get_stats(&self) -> DomainResult<HubStats> {
        self.skill_repo.get_stats().await
    }

    pub async fn list_my_skills(&self, page: u32, page_size: u32) -> DomainResult<(Vec<Skill>, i64)> {
        let result = self.skill_repo.list_my_skills(page, page_size).await?;
        Ok((result.skills, result.total))
    }

    pub async fn list_my_stars(&self, page: u32, page_size: u32) -> DomainResult<(Vec<Skill>, i64)> {
        let result = self.skill_repo.list_my_stars(page, page_size).await?;
        Ok((result.skills, result.total))
    }

    pub async fn star_skill(&self, skill_id: &str) -> DomainResult<()> {
        self.skill_repo.star(skill_id).await
    }

    pub async fn unstar_skill(&self, skill_id: &str) -> DomainResult<()> {
        self.skill_repo.unstar(skill_id).await
    }

    pub async fn rate_skill(&self, skill_id: &str, score: i16) -> DomainResult<()> {
        self.skill_repo.rate(skill_id, score).await
    }

    pub async fn download_bundle(
        &self,
        namespace: &str,
        slug: &str,
        version: &str,
    ) -> DomainResult<Vec<u8>> {
        self.skill_repo.download_bundle(namespace, slug, version).await
    }

    pub async fn download_latest(&self, namespace: &str, slug: &str) -> DomainResult<Vec<u8>> {
        self.skill_repo.download_latest(namespace, slug).await
    }

    // Namespace methods
    pub async fn list_namespaces(&self) -> DomainResult<Vec<Namespace>> {
        match &self.namespace_repo {
            Some(repo) => repo.list().await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    // Label methods
    pub async fn list_labels(&self) -> DomainResult<Vec<Label>> {
        match &self.label_repo {
            Some(repo) => repo.list().await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    // Review methods
    pub async fn list_reviews(&self, status: Option<&str>) -> DomainResult<Vec<ReviewTask>> {
        match &self.review_repo {
            Some(repo) => repo.list(status).await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    // Notification methods
    pub async fn list_notifications(
        &self,
        notification_type: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> DomainResult<Vec<Notification>> {
        match &self.notification_repo {
            Some(repo) => repo.list(notification_type, page, page_size).await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    pub async fn get_unread_notification_count(&self) -> DomainResult<i64> {
        match &self.notification_repo {
            Some(repo) => repo.get_unread_count().await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    pub async fn mark_notification_read(&self, id: &str) -> DomainResult<()> {
        match &self.notification_repo {
            Some(repo) => repo.mark_as_read(id).await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    pub async fn mark_all_notifications_read(&self) -> DomainResult<()> {
        match &self.notification_repo {
            Some(repo) => repo.mark_all_as_read().await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    pub async fn delete_notification(&self, id: &str) -> DomainResult<()> {
        match &self.notification_repo {
            Some(repo) => repo.delete(id).await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    pub async fn get_notification_preferences(&self) -> DomainResult<Vec<NotificationPreference>> {
        match &self.notification_repo {
            Some(repo) => repo.get_preferences().await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    // Token methods
    pub async fn list_tokens(&self, page: u32, page_size: u32) -> DomainResult<Vec<ApiToken>> {
        match &self.token_repo {
            Some(repo) => repo.list(page, page_size).await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    pub async fn delete_token(&self, id: &str) -> DomainResult<()> {
        match &self.token_repo {
            Some(repo) => repo.delete(id).await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    // User methods
    pub async fn get_user_profile(&self) -> DomainResult<Option<UserProfile>> {
        match &self.user_repo {
            Some(repo) => repo.get_profile().await,
            None => Err(DomainError::NotAuthenticated),
        }
    }

    // Auth methods
    pub async fn login(&self, username: &str, password: &str) -> DomainResult<String> {
        match &self.auth_repo {
            Some(repo) => repo.login(username, password).await,
            None => Err(DomainError::Config("Auth repository not configured".to_string())),
        }
    }

    pub async fn login_with_app(&self, app_id: &str, app_secret: &str) -> DomainResult<String> {
        match &self.auth_repo {
            Some(repo) => repo.login_with_app(app_id, app_secret).await,
            None => Err(DomainError::Config("Auth repository not configured".to_string())),
        }
    }
}
