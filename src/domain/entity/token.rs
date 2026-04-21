#[derive(Debug, Clone)]
pub struct ApiToken {
    pub id: String,
    pub name: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}
