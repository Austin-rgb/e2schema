#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdated {
    pub user_id: Uuid,

    /// Partial update pattern
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country: Option<String>,
}

use event_stream::Publishable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreated {
    pub user_id: Uuid,
    pub email: String,
    pub phone: Option<String>,
    pub country: Option<String>,
}

impl Publishable for UserCreated {
    const SUBJECT: &'static str = "user.user.created";
}

impl Publishable for UserUpdated {
    const SUBJECT: &'static str = "user.user.updated";
}
