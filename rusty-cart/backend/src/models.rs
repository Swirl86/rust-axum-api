use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: u32,
    pub title: String,
    pub price: f64,
    pub description: String,
    pub category: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
}

// Payload structs for cart operations
#[derive(serde::Deserialize)]
pub struct EditCartItemPayload {
    pub product_id: u32,
    pub quantity: u32,
}

#[derive(serde::Deserialize)]
pub struct DeleteCartItemPayload {
    pub product_id: u32,
}
