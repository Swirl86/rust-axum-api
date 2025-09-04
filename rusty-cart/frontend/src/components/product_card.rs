use yew::prelude::*;
use crate::models::Product;

#[derive(Properties, PartialEq, Clone)]
pub struct ProductCardProps {
    pub product: Product,
    pub on_add: Callback<Product>,
}

#[function_component(ProductCard)]
pub fn product_card(props: &ProductCardProps) -> Html {
    let product = props.product.clone();
    let onclick = {
        let on_add = props.on_add.clone();
        Callback::from(move |_| on_add.emit(product.clone()))
    };

    let scale = use_state(|| 1.0);
    let on_mouse_over = {
        let scale = scale.clone();
        Callback::from(move |_| scale.set(1.03))
    };
    let on_mouse_out = {
        let scale = scale.clone();
        Callback::from(move |_| scale.set(1.0))
    };

    let btn_hover = use_state(|| false);
    let on_btn_mouse_over = {
        let btn_hover = btn_hover.clone();
        Callback::from(move |_| btn_hover.set(true))
    };
    let on_btn_mouse_out = {
        let btn_hover = btn_hover.clone();
        Callback::from(move |_| btn_hover.set(false))
    };

    let btn_bg = if *btn_hover { "#45a049" } else { "#4CAF50" };

    html! {
        <div
            onmouseover={on_mouse_over}
            onmouseout={on_mouse_out}
            style={format!(
                "border: 1px solid #ddd;
                 border-radius: 10px;
                 padding: 16px;
                 box-shadow: 0 2px 6px rgba(0,0,0,0.1);
                 background: white;
                 display: flex;
                 flex-direction: column;
                 justify-content: space-between;
                 transition: transform 0.2s;
                 transform: scale({});",
                 *scale
            )}
        >
            <img
                src={props.product.image.clone()}
                alt={props.product.title.clone()}
                style="border-radius: 6px; object-fit: cover; width: 100%; height: 140px; margin-bottom: 12px;"
            />

            <h3 style="margin: 0 0 8px 0; font-size: 1.1em; font-weight: bold;">
                { &props.product.title }
            </h3>

            <p style="margin: 0 0 12px 0; font-size: 0.9em; color: #333;">
                { format!("{} kr", props.product.price) }
            </p>

            <button
                {onclick}
                onmouseover={on_btn_mouse_over}
                onmouseout={on_btn_mouse_out}
                style={format!(
                    "padding: 8px 12px;
                     border-radius: 6px;
                     border: none;
                     cursor: pointer;
                     background: {};
                     color: white;
                     font-weight: bold;
                     transition: background 0.2s;",
                     btn_bg
                )}
            >
                { "Add ➕" }
            </button>
        </div>
    }
}
