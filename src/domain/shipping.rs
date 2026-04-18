use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentUpdated {
    pub shipment_id: Uuid,
    pub status: ShipmentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShipmentStatus {
    Pending,
    InTransit,
    OutForDelivery,
    Delivered,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentCreated {
    pub shipment_id: Uuid,
    pub order_id: Uuid,
    pub carrier: String,
}
