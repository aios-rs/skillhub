pub mod auth_repository_impl;
pub mod skill_repository_impl;

use crate::infrastructure::client::SkillHubClient;
use std::sync::Arc;

pub type ClientRef = Arc<SkillHubClient>;
