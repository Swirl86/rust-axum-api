use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Product {
    pub id: u32,
    pub title: String,
    pub price: f64,
    pub description: String,
    pub category: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
}
