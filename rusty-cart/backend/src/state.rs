use std::sync::{Arc, Mutex};
use crate::models::CartItem;

#[derive(Clone)]
pub struct AppState {
    pub cart: Arc<Mutex<Vec<CartItem>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            cart: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
