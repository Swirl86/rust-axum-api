use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::models::CartItem;

#[derive(Properties, PartialEq, Clone)]
pub struct CartItemCardProps {
    pub item: CartItem,
    pub index: usize,
    pub on_edit: Callback<(u32, u32)>,
    pub on_delete: Callback<u32>,
}

#[function_component(CartItemCard)]
pub fn cart_item_card(props: &CartItemCardProps) -> Html {
    let item = props.item.clone();
    let on_edit = props.on_edit.clone();
    let on_delete = props.on_delete.clone();

    let editing = use_state(|| false);
    let input_value = use_state(|| item.quantity.to_string());

    let hover = use_state(|| false);
    let bg_color = if props.index % 2 == 0 { "#C5C6D0" } else { "#ADADC9" };

    let increase = {
        let on_edit = on_edit.clone();
        Callback::from(move |_| {
            on_edit.emit((item.product.id, item.quantity + 1));
        })
    };

    let decrease = {
        let on_edit = on_edit.clone();
        Callback::from(move |_| {
            if item.quantity > 1 {
                on_edit.emit((item.product.id, item.quantity - 1));
            }
        })
    };

    let delete_item = {
        let on_delete = on_delete.clone();
        let product_id = item.product.id;
        Callback::from(move |_| {
            on_delete.emit(product_id);
        })
    };

    let save_value = {
        let editing = editing.clone();
        let input_value = input_value.clone();
        let on_edit = on_edit.clone();
        let product_id = item.product.id;
        Callback::from(move |_: FocusEvent| {
            editing.set(false);
            if let Ok(val) = input_value.parse::<u32>() {
                on_edit.emit((product_id, val));
            } else {
                input_value.set("1".to_string());
                on_edit.emit((product_id, 1));
            }
        })
    };

    let on_keydown = {
        let editing = editing.clone();
        let input_value = input_value.clone();
        let on_edit = on_edit.clone();
        let product_id = item.product.id;

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" || e.key() == "Tab" {
                editing.set(false);
                if let Ok(val) = input_value.parse::<u32>() {
                    on_edit.emit((product_id, val));
                } else {
                    input_value.set("1".to_string());
                    on_edit.emit((product_id, 1));
                }
            }
        })
    };


    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let val = input.value();
                if val.chars().all(|c| c.is_ascii_digit()) {
                    input_value.set(val);
                }
            }
        })
    };

    html! {
        <li style={format!(
            "display: flex;
            align-items: center;
            justify-content: space-between;
            margin: 6px 0;
            padding: 8px;
            border-bottom: 1px solid #ddd;
            background-color: {};",
            bg_color
        )}>
            <span style="flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                { &item.product.title }
            </span>

            <div style="
                display: flex;
                align-items: center;
                gap: 12px;
                width: 150px;
                justify-content: center;
                font-weight: bold;
            ">
                <button onclick={decrease.clone()} style="font-weight: bold; padding: 6px 10px;">{"-"}</button>

                if *editing {
                    <input
                        type="text"
                        value={(*input_value).clone()}
                        oninput={on_input.clone()}
                        onblur={save_value.clone()}
                        onkeydown={on_keydown.clone()}
                        style="width: 50px; text-align: center; font-weight: bold; border-radius: 4px; border: 1px solid #ccc;"
                        autofocus=true
                    />
                } else {
                    <span
                        onclick={
                            let editing = editing.clone();
                            Callback::from(move |_| editing.set(true))
                        }
                        onmouseover={
                            let hover = hover.clone();
                            Callback::from(move |_| hover.set(true))
                        }
                        onmouseout={
                            let hover = hover.clone();
                            Callback::from(move |_| hover.set(false))
                        }
                        style={format!(
                            "cursor: pointer; font-weight: bold; padding: 2px 6px; border-radius: 4px; background-color: {};",
                            if *hover { "#e0e0e0" } else { "transparent" }
                        )}
                        title="Edit quantity"
                    >
                        { item.quantity }
                    </span>
                }

                <button onclick={increase.clone()} style="font-weight: bold; padding: 6px 10px;">{"+"}</button>
            </div>

            <span style="
                width: 100px;
                text-align: center;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            ">
                { format!("= {:.2} kr", item.product.price * item.quantity as f64) }
            </span>

            <span
                onclick={delete_item}
                style="cursor: pointer; margin-left: 8px; color: red;"
                title="Remove item"
            >
                { "🗑️" }
            </span>
        </li>
    }
}
