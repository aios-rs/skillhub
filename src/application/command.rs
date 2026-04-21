use crate::domain::value_object::visibility::Visibility;

#[derive(Debug, Clone)]
pub struct SearchSkillsCommand {
    pub query: Option<String>,
    pub namespace: Option<String>,
    pub labels: Vec<String>,
    pub visibility: Option<String>,
    pub sort: String,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone)]
pub struct PublishSkillCommand {
    pub namespace: String,
    pub file_path: String,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct RateSkillCommand {
    pub skill_id: String,
    pub score: i16,
}

#[derive(Debug, Clone)]
pub struct CreateNamespaceCommand {
    pub slug: String,
    pub display_name: String,
    pub namespace_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateTokenCommand {
    pub name: String,
    pub scopes: Vec<String>,
    pub expiration_mode: Option<String>,
    pub custom_expires_at: Option<String>,
}
