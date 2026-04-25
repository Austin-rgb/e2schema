use event_stream::Publishable;
use serde::{Deserialize, Serialize};
use crate::EventMetaData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
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
    pub product_id: String,
    pub _emd: EventMetaData,
    pub sku: String,
    pub name: String,
    pub category_id: String,
    pub price: Money,
    pub attributes: Option<ProductAttributes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductUpdated {
    pub product_id: String,
    pub _emd: EventMetaData,
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub price: Option<Money>,
    pub attributes: Option<ProductAttributes>,

    pub updated_fields: Vec<String>,
}

impl Publishable for ProductCreated {
    const SUBJECT: &'static str = "catalog.product.created";
}

impl Publishable for ProductUpdated {
    const SUBJECT: &'static str = "catalog.product.updated";
}
