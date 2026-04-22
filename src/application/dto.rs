use serde::{Deserialize, Serialize};

// ===== Auth =====

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: LoginUserDto,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginUserDto {
    pub avatar: Option<String>,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

// ===== Search & Skill =====

#[derive(Debug, Clone, Deserialize)]
pub struct HubSkillDto {
    pub id: String,
    pub tenant_id: String,
    pub namespace_id: String,
    pub namespace_slug: String,
    pub slug: String,
    pub display_name: Option<String>,
    pub summary: Option<String>,
    pub owner_name: String,
    pub owner_id: String,
    pub visibility: String,
    pub status: String,
    pub download_count: i64,
    pub star_count: i32,
    pub rating_avg: f64,
    pub rating_count: i32,
    pub hidden: bool,
    pub tags: Vec<String>,
    pub labels: Vec<LabelDto>,
    pub can_manage_lifecycle: bool,
    pub can_submit_promotion: bool,
    pub can_interact: bool,
    pub can_report: bool,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub latest_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HubSkillVersionDto {
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

#[derive(Debug, Clone, Deserialize)]
pub struct HubSkillFileDto {
    pub id: String,
    pub version_id: String,
    pub file_path: String,
    pub file_size: i64,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LabelDto {
    pub id: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub label_type: String,
    pub display_name: String,
    pub visible_in_filter: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HubSearchResultDto {
    pub skills: Vec<HubSkillDto>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HubStatsDto {
    pub total_skills: i64,
    pub total_downloads: i64,
    pub total_namespaces: i32,
    pub total_ratings: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishResultDto {
    pub skill_id: String,
    pub namespace: String,
    pub slug: String,
    pub version: String,
    pub status: String,
    pub file_count: i32,
    pub total_size: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkillLifecycleMutationResponseDto {
    pub skill_id: String,
    pub version_id: Option<String>,
    pub action: String,
    pub new_status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkillSummaryDto {
    pub id: String,
    pub namespace_slug: String,
    pub slug: String,
    pub display_name: Option<String>,
    pub summary: Option<String>,
    pub visibility: String,
    pub status: String,
    pub star_count: i32,
    pub download_count: i64,
    pub rating_avg: f64,
    pub rating_count: i32,
    pub updated_at: String,
    pub latest_version: Option<String>,
}

// ===== Namespace =====

#[derive(Debug, Clone, Deserialize)]
pub struct HubNamespaceDto {
    pub id: String,
    pub slug: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub namespace_type: String,
    pub description: Option<String>,
    pub status: String,
    pub member_count: i32,
    pub skill_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NamespaceMemberDto {
    pub id: String,
    pub namespace_id: String,
    pub user_id: String,
    pub role: String,
    pub created_at: String,
}

// ===== Review =====

#[derive(Debug, Clone, Deserialize)]
pub struct ReviewTaskDto {
    pub id: String,
    pub skill_id: String,
    pub submitter_id: String,
    pub status: String,
    pub comment: Option<String>,
    pub created_at: String,
}

// ===== Report =====

#[derive(Debug, Clone, Deserialize)]
pub struct SkillReportDto {
    pub id: String,
    pub skill_id: String,
    pub reporter_id: String,
    pub reason: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
}

// ===== Promotion =====

#[derive(Debug, Clone, Deserialize)]
pub struct PromotionDto {
    pub id: String,
    pub source_skill_id: i64,
    pub source_version_id: i64,
    pub target_namespace_id: i64,
    pub status: String,
    pub comment: Option<String>,
    pub created_at: String,
}

// ===== Notification =====

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationDto {
    pub id: i64,
    pub user_id: String,
    pub notification_type: String,
    pub title: String,
    pub content: Option<String>,
    pub data: Option<serde_json::Value>,
    pub read_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationPreferenceDto {
    pub id: i64,
    pub user_id: String,
    pub notification_type: String,
    pub enabled: bool,
    pub channel: String,
    pub updated_at: String,
}

// ===== Token =====

#[derive(Debug, Clone, Deserialize)]
pub struct ApiTokenDto {
    pub id: String,
    pub name: String,
    pub token_prefix: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
    pub last_used_at: Option<String>,
    pub created_at: String,
}

// ===== User =====

#[derive(Debug, Clone, Deserialize)]
pub struct UserProfileDto {
    pub id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileChangeRequestDto {
    pub id: String,
    pub user_id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub created_at: String,
}

// ===== Generic API Response =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiReply<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiReply<T> {
    pub fn into_result(self) -> Result<T, String> {
        if self.code == 0 {
            self.data.ok_or_else(|| "Empty response data".to_string())
        } else {
            Err(format!("API error (code {}): {}", self.code, self.msg))
        }
    }
}
