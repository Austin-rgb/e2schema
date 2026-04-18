use serde::{Serialize,Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAbandoned {
    pub cart_id: Uuid,
    pub user_id: Uuid,
    pub last_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductViewed {
    pub product_id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReviewed {
    pub review_id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: u8, // 1-5
    pub comment: Option<String>,
}
