#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: String,
    pub slug: String,
    pub display_name: String,
    pub namespace_type: String,
    pub description: Option<String>,
    pub status: String,
    pub member_count: i32,
    pub skill_count: i32,
}

#[derive(Debug, Clone)]
pub struct NamespaceMember {
    pub id: String,
    pub namespace_id: String,
    pub user_id: String,
    pub role: String,
    pub created_at: String,
}
