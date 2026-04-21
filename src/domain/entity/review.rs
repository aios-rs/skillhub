#[derive(Debug, Clone)]
pub struct ReviewTask {
    pub id: String,
    pub skill_id: String,
    pub submitter_id: String,
    pub status: String,
    pub comment: Option<String>,
    pub created_at: String,
}
