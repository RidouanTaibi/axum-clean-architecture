use uuid::Uuid;

#[derive(Debug)]
pub struct Page {
    pub id: Uuid,
    pub uuid: Uuid,
    pub title: String,
    pub type_code: String,
    pub template: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
