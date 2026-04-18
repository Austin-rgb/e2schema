#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdated {
    pub user_id: Uuid,

    /// Partial update pattern
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country: Option<String>,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreated {
    pub user_id: Uuid,
    pub email: String,
    pub phone: Option<String>,
    pub country: Option<String>,
}
