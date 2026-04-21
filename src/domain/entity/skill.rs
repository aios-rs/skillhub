use crate::domain::value_object::visibility::Visibility;

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: String,
    pub tenant_id: String,
    pub namespace_id: String,
    pub namespace_slug: String,
    pub slug: String,
    pub display_name: Option<String>,
    pub summary: Option<String>,
    pub owner_name: String,
    pub owner_id: String,
    pub visibility: Visibility,
    pub status: String,
    pub download_count: i64,
    pub star_count: i32,
    pub rating_avg: f64,
    pub rating_count: i32,
    pub hidden: bool,
    pub tags: Vec<String>,
    pub labels: Vec<SkillLabel>,
    pub can_manage_lifecycle: bool,
    pub can_submit_promotion: bool,
    pub can_interact: bool,
    pub can_report: bool,
    pub created_at: String,
    pub updated_at: String,
    pub latest_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SkillLabel {
    pub id: String,
    pub slug: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct SkillVersion {
    pub id: String,
    pub skill_id: String,
    pub version: String,
    pub status: String,
    pub changelog: Option<String>,
    pub file_count: i32,
    pub total_size: i64,
    pub published_at: Option<String>,
    pub bundle_ready: bool,
    pub download_ready: bool,
    pub create_by: String,
    pub create_at: String,
}

#[derive(Debug, Clone)]
pub struct SkillFile {
    pub id: String,
    pub version_id: String,
    pub file_path: String,
    pub file_size: i64,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct HubStats {
    pub total_skills: i64,
    pub total_downloads: i64,
    pub total_namespaces: i32,
    pub total_ratings: i64,
}

#[derive(Debug, Clone)]
pub struct PublishResult {
    pub skill_id: String,
    pub namespace: String,
    pub slug: String,
    pub version: String,
    pub status: String,
    pub file_count: i32,
    pub total_size: i64,
}

#[derive(Debug, Clone)]
pub struct SkillLifecycleMutationResponse {
    pub skill_id: String,
    pub version_id: Option<String>,
    pub action: String,
    pub new_status: String,
}
