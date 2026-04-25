use crate::EventMetaData;
use event_stream::Publishable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockAdjusted {
    pub product_id: Uuid,
    pub _emd: EventMetaData,
    pub previous_quantity: i64,
    pub new_quantity: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReserved {
    pub product_id: Uuid,
    pub _emd: EventMetaData,
    pub order_id: Uuid,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReleased {
    pub product_id: Uuid,
    pub _emd: EventMetaData,
    pub order_id: Uuid,
    pub quantity: u32,
}

impl Publishable for StockAdjusted {
    const SUBJECT: &'static str = "inventory.stock.adjusted";
}

impl Publishable for InventoryReserved {
    const SUBJECT: &'static str = "inventory.stock.reserved";
}

impl Publishable for InventoryReleased {
    const SUBJECT: &'static str = "inventory.stock.released";
}
