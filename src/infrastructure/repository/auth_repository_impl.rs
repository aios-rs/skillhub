use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::error::{DomainError, DomainResult};
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
}
