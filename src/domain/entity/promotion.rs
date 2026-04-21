#[derive(Debug, Clone)]
pub struct Promotion {
    pub id: String,
    pub source_skill_id: String,
    pub source_version_id: String,
    pub target_namespace_id: String,
    pub status: String,
    pub comment: Option<String>,
    pub created_at: String,
}
