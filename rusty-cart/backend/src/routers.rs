use axum::{extract::State, Json, http::StatusCode};
use crate::models::{Product, CartItem, EditCartItemPayload, DeleteCartItemPayload};
use crate::state::AppState;
use serde_json::{json, Value};

// Fetch products from Fake Store API
pub async fn get_products() -> Result<Json<Vec<Product>>, (StatusCode, Json<Value>)> {
    let url = "https://fakestoreapi.com/products";
    let res = reqwest::get(url).await.map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": format!("Failed to fetch products: {}", e) }))
    ))?;

    let products: Vec<Product> = res.json().await.map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": format!("Failed to parse products: {}", e) }))
    ))?;

    //println!("Fetched products: {:#?}", products);
    Ok(Json(products))
}

pub async fn get_cart(State(state): State<AppState>) -> Result<Json<Vec<CartItem>>, (StatusCode, Json<serde_json::Value>)> {
    match state.cart.lock() {
        Ok(cart) => Ok(Json(cart.clone())),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to access cart: {}", e) }))
        )),
    }
}

pub async fn add_to_cart(
    State(state): State<AppState>,
    Json(product): Json<Product>,
) -> Json<serde_json::Value> {
    let mut cart = state.cart.lock().unwrap();

    if let Some(item) = cart.iter_mut().find(|i| i.product.id == product.id) {
        item.quantity += 1;
    } else {
        cart.push(CartItem { product, quantity: 1 });
    }

    Json(json!({ "status": "added to cart" }))
}

// Edit quantity of a product in the cart
pub async fn edit_cart_item(
    State(state): State<AppState>,
    Json(payload): Json<EditCartItemPayload>,
) -> Json<serde_json::Value> {
    let mut cart = state.cart.lock().unwrap();
    if let Some(item) = cart.iter_mut().find(|i| i.product.id == payload.product_id) {
        item.quantity = payload.quantity;
        Json(json!({ "status": "quantity updated" }))
    } else {
        Json(json!({ "error": "Product not found in cart" }))
    }
}

// Delete a product from the cart
pub async fn delete_cart_item(
    State(state): State<AppState>,
    Json(payload): Json<DeleteCartItemPayload>,
) -> Json<serde_json::Value> {
    let mut cart = state.cart.lock().unwrap();
    let len_before = cart.len();
    cart.retain(|item| item.product.id != payload.product_id);
    if cart.len() < len_before {
        Json(json!({ "status": "deleted from cart" }))
    } else {
        Json(json!({ "error": "Product not found in cart" }))
    }
}

