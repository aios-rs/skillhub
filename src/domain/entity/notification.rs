#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub notification_type: String,
    pub title: String,
    pub content: Option<String>,
    pub data: Option<String>,
    pub read: bool,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct NotificationPreference {
    pub notification_type: String,
    pub enabled: bool,
}
