use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSent {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub channel: NotificationChannel,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
}

