#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct ProfileChangeRequest {
    pub id: String,
    pub user_id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub created_at: String,
}
