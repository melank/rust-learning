use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub body: String,
}

impl CreateNoteRequest {
    pub fn is_valid(&self) -> bool {
        !self.title.trim().is_empty() && !self.body.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::CreateNoteRequest;

    #[test]
    fn rejects_empty_payload() {
        let payload = CreateNoteRequest {
            title: String::new(),
            body: String::from("body"),
        };
        assert!(!payload.is_valid());
    }
}
