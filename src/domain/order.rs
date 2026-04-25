use event_stream::Publishable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::EventMetaData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub quantity: u32,
    pub price: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPlaced {
    pub order_id: Uuid,
    pub _emd: EventMetaData,
    pub user_id: Uuid,
    pub items: Vec<OrderItem>,
    pub total_amount: i64,
    pub currency: String,
}

impl Publishable for OrderPlaced {
    const SUBJECT: &'static str = "orders.order.placed";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelled {
    pub order_id: Uuid,
    pub _emd: EventMetaData,
    pub reason: Option<String>,
}

impl Publishable for OrderCancelled {
    const SUBJECT: &'static str = "orders.order.cancelled";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderConfirmed {
    pub order_id: Uuid,
    pub _emd: EventMetaData,
    pub payment_id: Uuid,
}

impl Publishable for OrderConfirmed {
    const SUBJECT: &'static str = "orders.order.confirmed";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderShipped {
    pub order_id: Uuid,
    pub _emd: EventMetaData,
    pub shipment_id: Uuid,
    pub carrier: String,
    pub tracking_number: String,
}

impl Publishable for OrderShipped {
    const SUBJECT: &'static str = "orders.order.shipped";
}
