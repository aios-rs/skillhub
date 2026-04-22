#[derive(Debug, Clone)]
pub struct Notification {
    pub id: i64,
    pub user_id: String,
    pub notification_type: String,
    pub title: String,
    pub content: Option<String>,
    pub data: Option<serde_json::Value>,
    pub read_at: Option<String>,
    pub created_at: String,
}

impl Notification {
    pub fn is_read(&self) -> bool {
        self.read_at.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct NotificationPreference {
    pub id: i64,
    pub user_id: String,
    pub notification_type: String,
    pub enabled: bool,
    pub channel: String,
    pub updated_at: String,
}
