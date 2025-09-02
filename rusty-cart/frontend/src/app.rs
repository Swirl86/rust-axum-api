use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use gloo::console::{log, error};

use crate::models::{Product, CartItem};

/// Base URL
const BACKEND_URL: &str = "http://127.0.0.1:3000";

/// Endpoints
const PRODUCTS_ENDPOINT: &str = "/products";
const CART_ENDPOINT: &str = "/cart";
const ADD_TO_CART_ENDPOINT: &str = "/cart/add";

async fn fetch_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    match Request::get(url).send().await {
        Ok(resp) if resp.ok() => resp
            .json::<T>()
            .await
            .map_err(|e| format!("JSON parse error: {:?}", e)),
        Ok(resp) => {
            let text = resp.text().await.unwrap_or_default();
            Err(format!("Server error: {} -> {}", resp.status(), text))
        }
        Err(e) => Err(format!("Network error: {:?}", e)),
    }
}

#[function_component(RustyCart)]
pub fn rusty_cart() -> Html {
    let products = use_state(|| Vec::<Product>::new());
    let cart = use_state(|| Vec::<CartItem>::new());

    // Fetch products
    {
        let products = products.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let url = format!("{BACKEND_URL}{PRODUCTS_ENDPOINT}");
                match fetch_json::<Vec<Product>>(&url).await {
                    Ok(data) => {
                        log!(format!("Fetched {} products", data.len()));
                        products.set(data);
                    }
                    Err(err) => error!(err),
                }
            });
            || ()
        });
    }

    // Fetch cart
    {
        let cart = cart.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let url = format!("{BACKEND_URL}{CART_ENDPOINT}");
                match fetch_json::<Vec<CartItem>>(&url).await {
                    Ok(data) => {
                        log!(format!("Fetched cart with {} items", data.len()));
                        cart.set(data);
                    }
                    Err(err) => error!(err),
                }
            });
            || ()
        });
    }

    let add_to_cart = {
        let cart = cart.clone();
        move |product: Product| {
            let cart = cart.clone();
            spawn_local(async move {
                let url = format!("{BACKEND_URL}{ADD_TO_CART_ENDPOINT}");
                let req = Request::post(&url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&product).unwrap())
                    .expect("Failed to build request");

                match req.send().await {
                    Ok(resp) if resp.ok() => {
                        cart.set({
                            let mut new_cart = (*cart).clone();
                            if let Some(item) = new_cart.iter_mut().find(|i| i.product.id == product.id) {
                                item.quantity += 1;
                            } else {
                                new_cart.push(CartItem {
                                    product: product.clone(),
                                    quantity: 1,
                                });
                            }
                            new_cart
                        });
                    }
                    Ok(resp) => error!(format!("Add to cart failed: {}", resp.status())),
                    Err(err) => error!(format!("Request error: {:?}", err)),
                }
            });
        }
    };

    // Calculate total
    let total: f64 = (*cart)
        .iter()
        .map(|item| item.product.price as f64 * item.quantity as f64)
        .sum();

    html! {
        <div>
            <h1>{ "🦀 RustyCart" }</h1>

            <h2>
                { "Products " }
                <span style="font-size: 0.8em; color: gray;">
                    { format!("({} available)", products.len()) }
                </span>
            </h2>

            <ul style="list-style: none; padding: 0;">
                { for (*products).iter().map(|product| {
                    let product_clone = product.clone();
                    let onclick = {
                        let add_to_cart = add_to_cart.clone();
                        Callback::from(move |_| add_to_cart(product_clone.clone()))
                    };
                    html! {
                        <li style="margin: 8px 0; display: flex; align-items: center; gap: 12px;">
                            <img src={product.image.clone()} width="50" style="border-radius: 4px;"/>
                            <span style="flex: 1;">
                                { format!("{} - {} kr", product.title, product.price) }
                            </span>
                            <button {onclick} style="padding: 4px 10px; border-radius: 6px; cursor: pointer;">
                                { "Add ➕" }
                            </button>
                        </li>
                    }
                }) }
            </ul>

            <h2>{ "🛒 Your order" }</h2>
            <ul style="list-style: none; padding: 0;">
                { for (*cart).iter().map(|item| html! {
                    <li style="margin: 6px 0;">
                        { format!("{} x {} = {} kr",
                            item.product.title,
                            item.quantity,
                            item.product.price * item.quantity as f64
                        )}
                    </li>
                }) }
            </ul>

            <h3 style="margin-top: 16px;">
                { format!("💰 Total: {} kr", total) }
            </h3>
        </div>
    }
}
