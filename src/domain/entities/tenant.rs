use uuid::Uuid;

#[derive(Debug)]
pub struct Tenant {
    pub id: Uuid,
    pub uuid: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}