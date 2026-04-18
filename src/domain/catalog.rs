use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAttributes {
    pub color: Option<String>,
    pub size: Option<String>,
    pub material: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCreated {
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub category_id: Uuid,
    pub price: Money,
    pub attributes: Option<ProductAttributes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductUpdated {
    pub product_id: Uuid,

    pub name: Option<String>,
    pub category_id: Option<Uuid>,
    pub price: Option<Money>,
    pub attributes: Option<ProductAttributes>,

    pub updated_fields: Vec<String>,
}
