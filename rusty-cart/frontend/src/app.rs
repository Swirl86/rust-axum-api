use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use gloo::console::{log, error};

use crate::models::{Product, CartItem, EditCartItemPayload};
use crate::components::{ProductCard, CartItemCard};

const BACKEND_URL: &str = "http://127.0.0.1:3000";
const PRODUCTS_ENDPOINT: &str = "/products";
const CART_ENDPOINT: &str = "/cart";
const ADD_TO_CART_ENDPOINT: &str = "/cart/add";
const EDIT_CART_ENDPOINT: &str = "/cart/edit";

async fn fetch_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    match Request::get(url).send().await {
        Ok(resp) if resp.ok() => resp.json::<T>().await.map_err(|e| format!("JSON parse error: {:?}", e)),
        Ok(resp) => {
            let text = resp.text().await.unwrap_or_default();
            Err(format!("Server error {}: {}", resp.status(), text))
        }
        Err(e) => Err(format!("Network error: {:?}", e)),
    }
}

#[function_component(RustyCart)]
pub fn rusty_cart() -> Html {
    let products = use_state(|| Vec::<Product>::new());
    let cart = use_state(|| Vec::<CartItem>::new());
    let show_cart = use_state(|| false);

    let hover = use_state(|| false);
    let on_mouse_over = {
        let hover = hover.clone();
        Callback::from(move |_| hover.set(true))
    };
    let on_mouse_out = {
        let hover = hover.clone();
        Callback::from(move |_| hover.set(false))
    };

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
                    Err(err) => error!(format!("Failed to fetch products: {}", err)),
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
                    Err(err) => error!(format!("Failed to fetch cart: {}", err)),
                }
            });
            || ()
        });
    }

    let toggle_cart = {
        let show_cart = show_cart.clone();
        Callback::from(move |_| show_cart.set(!*show_cart))
    };

    let add_to_cart = {
        let cart = cart.clone();
        move |product: Product| {
            let cart = cart.clone();
            spawn_local(async move {
                let url = format!("{BACKEND_URL}{ADD_TO_CART_ENDPOINT}");
                let req = Request::post(&url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&product).unwrap())
                    .unwrap();

                match req.send().await {
                    Ok(resp) if resp.ok() => {
                        log!(format!("Added product {} to cart", product.title));
                        cart.set({
                            let mut new_cart = (*cart).clone();
                            if let Some(item) = new_cart.iter_mut().find(|i| i.product.id == product.id) {
                                item.quantity += 1;
                            } else {
                                new_cart.push(CartItem { product: product.clone(), quantity: 1 });
                            }
                            new_cart
                        });
                    }
                    Ok(resp) => error!(format!("Failed to add product {}: server returned {}", product.title, resp.status())),
                    Err(err) => error!(format!("Failed to add product {}: {:?}", product.title, err)),
                }
            });
        }
    };

    // Edit cart item quantity
    let edit_cart_item = {
        let cart = cart.clone();
        move |(product_id, quantity): (u32, u32)| {
            let cart = cart.clone();
            spawn_local(async move {
                let payload = EditCartItemPayload { product_id, quantity };
                let req = Request::post(&format!("{BACKEND_URL}{EDIT_CART_ENDPOINT}"))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .unwrap();

                match req.send().await {
                    Ok(resp) if resp.ok() => {
                        log!(format!("Updated product {} quantity to {}", product_id, quantity));
                        cart.set({
                            let mut new_cart = (*cart).clone();
                            if let Some(item) = new_cart.iter_mut().find(|i| i.product.id == product_id) {
                                item.quantity = quantity;
                            }
                            new_cart
                        });
                    }
                    Ok(resp) => error!(format!("Failed to update product {}: server returned {}", product_id, resp.status())),
                    Err(err) => error!(format!("Failed to update product {}: {:?}", product_id, err)),
                }
            });
        }
    };

    let total: f64 = (*cart).iter().map(|item| item.product.price * item.quantity as f64).sum();

    html! {
        <div style="max-width: 900px; margin: auto; padding: 20px;">

            <h1 style="margin-bottom: 24px; font-size: 2em;">{ "🦀 RustyCart" }</h1>

            <h2 style="margin-bottom: 16px;">
                { format!("Products ({} available)", products.len()) }
            </h2>
            <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 16px;">
                { for (*products).iter().map(|product| {
                    html! {
                        <ProductCard product={product.clone()} on_add={add_to_cart.clone()} />
                    }
                }) }
            </div>

           <div
                onclick={toggle_cart.clone()}
                onmouseover={on_mouse_over.clone()}
                onmouseout={on_mouse_out.clone()}
                style={format!(
                    "margin-top: 30px;
                    padding: 12px 16px;
                    background: {};
                    border-radius: 8px;
                    font-weight: bold;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    cursor: pointer;",
                    if *hover { "#e0e0e0" } else { "#f5f5f5" }
                )}
            >
                <span>{ format!("🛒 {} items", cart.len()) }</span>
                <span>{ format!("Total: {:.2} kr", total) }</span>
            </div>

            if *show_cart {
                <div style="
                    margin-top: 12px;
                    padding: 12px;
                    background: #fafafa;
                    border: 1px solid #ddd;
                    border-radius: 8px;
                ">
                    {
                        if cart.is_empty() {
                            html! { <p>{ "Your cart is empty 🛒" }</p> }
                        } else {
                            html! {
                                <ul style="list-style: none; padding: 0; margin: 0;">
                                    { for (*cart).iter().enumerate().map(|(i, item)| {
                                        let edit_cart_item = edit_cart_item.clone();
                                        html! {
                                            <CartItemCard item={item.clone()} index={i} on_edit={edit_cart_item.clone()} />
                                        }
                                    }) }
                                </ul>
                            }
                        }
                    }
                </div>
            }
        </div>
    }
}
