use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Tick,
    ApiCall(ApiCall),
    ApiResult(ApiResult),
}

#[derive(Debug, Clone)]
pub enum ApiCall {
    Search(String),
    GetSkillDetail(String, String),
    ListVersions(String, String),
    Star(String),
    Unstar(String),
    Rate(String, i16),
    Download(String, String, String),
    GetStats,
    ListNamespaces,
    ListLabels,
    ListMySkills,
    ListMyStars,
    ListNotifications,
    MarkNotificationRead(String),
}

#[derive(Debug, Clone)]
pub enum ApiResult {
    Search(Result<(Vec<crate::domain::entity::skill::Skill>, i64), String>),
    SkillDetail(Option<crate::domain::entity::skill::Skill>),
    Versions(Vec<crate::domain::entity::skill::SkillVersion>),
    Star(()),
    Unstar(()),
    Rate(()),
    Download(Vec<u8>),
    Stats(crate::domain::entity::skill::HubStats),
    Namespaces(Vec<crate::domain::entity::namespace::Namespace>),
    Labels(Vec<crate::domain::entity::label::Label>),
    MySkills(Vec<crate::domain::entity::skill::Skill>),
    MyStars(Vec<crate::domain::entity::skill::Skill>),
    Notifications(Vec<crate::domain::entity::notification::Notification>),
    NotificationCount(i64),
    LoginSuccess,
    Error(String),
}
