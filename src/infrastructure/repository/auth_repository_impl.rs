use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::error::DomainResult;
use crate::domain::repository::auth_repository::AuthRepository;
use crate::infrastructure::client::SkillHubClient;

pub struct AuthRepositoryImpl {
    client: Arc<SkillHubClient>,
}

impl AuthRepositoryImpl {
    pub fn new(client: Arc<SkillHubClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn login(&self, username: &str, password: &str) -> DomainResult<String> {
        self.client.login(username, password).await
    }

    async fn login_with_app(&self, app_id: &str, app_secret: &str) -> DomainResult<String> {
        self.client.login_with_app(app_id, app_secret).await
    }
}
