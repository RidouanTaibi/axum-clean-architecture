use uuid:Uuid;

#[derive(Debug)]
pub struc User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}