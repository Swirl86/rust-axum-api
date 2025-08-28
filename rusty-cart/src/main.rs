use axum::{routing::{get, post}, Router};

use state::AppState;
use routers::{get_products, get_cart, add_to_cart, edit_cart_item, delete_cart_item};

mod models;
mod routers;
mod state;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    let app = Router::new()
        .route("/products", get(get_products))
        .route("/cart", get(get_cart))
        .route("/cart/add", post(add_to_cart))
        .route("/cart/edit", post(edit_cart_item))
        .route("/cart/delete", post(delete_cart_item))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port");

    println!("Server running at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}
