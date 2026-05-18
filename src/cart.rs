use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartCreated {
    pub cart_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemRemoved {
    pub cart_id: Uuid,
    pub product_id: Uuid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemAdded {
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartUpdatedQuantity {
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartCheckedOut {
    pub cart_id: Uuid,
    pub user_id: Uuid,
    pub total_amount: i64,
    pub currency: String,
}
