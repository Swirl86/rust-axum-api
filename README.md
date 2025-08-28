# Rusty Cart

Rusty Cart will be a simple e-commerce backend API built with Rust and Axum. It will fetch products from an external API and let you add them to an in-memory cart. This project is my first Rust project, created to learn Rust and web backend development.

⚡ Features (planned)
  - ```GET /products``` – Fetch products from Fake Store API.
  - ```GET /cart``` – View items in your cart.
  - ```POST /cart/add``` – Add a product to the cart (increases quantity if already added).
Data is stored in memory and resets when the server restarts.

🌱 Learning Goals
This project is planned to help me:
  - Learn Rust syntax, ownership, and async programming.
  - Build a simple HTTP API with Axum.
  - Handle external API data (Fake Store API) and manage in-memory state.
  - Gradually expand to database persistence, checkout flow, and frontend integration.

🚀 Run Locally
```
git clone https://github.com/Swirl86/rust-axum-api.git
cd rusty-cart
cargo run
```
Server runs at ```http://127.0.0.1:3000```.
