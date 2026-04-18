use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiated {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub method: PaymentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Card,
    Mpesa,
    PayPal,
    BankTransfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFailed {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub reason: PaymentFailureReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentFailureReason {
    InsufficientFunds,
    Declined,
    Timeout,
    FraudSuspected,
    ProviderError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSucceeded {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub transaction_id: String,
    pub provider: String,
}
