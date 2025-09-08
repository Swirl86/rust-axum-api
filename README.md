# Rusty Cart

Rusty Cart will be a simple e-commerce backend API built with Rust and Axum. It will fetch products from an external API and let you add them to an in-memory cart. This project is my first Rust project, created to learn Rust and web backend development.


⚡ Features
  - **Backend API**
    - `GET /products` – Fetch products from Fake Store API.
    - `GET /cart` – View items in your cart.
    - `POST /cart/add` – Add a product to the cart (increases quantity if already added).
    - `POST /cart/edit` – Edit the quantity of a product in the cart.
    - `POST /cart/delete` – Remove a product from the cart.
  - **Frontend**
    - View all products and add them to the cart.
    - Show cart with product quantities and total prices.
    - Edit quantities directly in the cart using +/- buttons or input field.
    - Dynamic UI: alternating row colors, hover effects, and formatted prices.

🌱 Learning Goals
This project helps to:
  - Learn Rust syntax, ownership, and async programming.
  - Build a simple HTTP API with Axum.
  - Manage in-memory state and integrate with a frontend.
  - Explore Yew for Rust-based frontend development.
  - Implement user interactions like editing cart items with instant feedback.

🚀 Run Locally
```bash
git clone https://github.com/Swirl86/rust-axum-api.git
cd rusty-cart
```

**Backend**
```bash
cd backend
cargo run
```
Server runs at ```http://127.0.0.1:3000```.

**Backend**
```bash
cd frontend
trunk serve
```
Open your browser at ```http://127.0.0.1:8080```.

**Important:** Make sure to start the backend server before starting the frontend, otherwise API requests will fail.

