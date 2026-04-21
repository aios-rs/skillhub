#[derive(Debug, Clone)]
pub struct SkillReport {
    pub id: String,
    pub skill_id: String,
    pub reporter_id: String,
    pub reason: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
}
