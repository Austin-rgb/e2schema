use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAdjusted {
    pub product_id: Uuid,
    pub previous_quantity: i64,
    pub new_quantity: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReserved {
    pub product_id: Uuid,
    pub order_id: Uuid,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReleased {
    pub product_id: Uuid,
    pub order_id: Uuid,
    pub quantity: u32,
}
