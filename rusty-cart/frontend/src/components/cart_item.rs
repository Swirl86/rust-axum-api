use yew::prelude::*;
use crate::models::CartItem;

#[derive(Properties, PartialEq, Clone)]
pub struct CartItemCardProps {
    pub item: CartItem,
}

#[function_component(CartItemCard)]
pub fn cart_item_card(props: &CartItemCardProps) -> Html {
    html! {
        <li style="margin: 6px 0; padding: 8px; border-bottom: 1px solid #ddd;">
            { format!("{} x {} = {} kr",
                props.item.product.title,
                props.item.quantity,
                props.item.product.price * props.item.quantity as f64
            )}
        </li>
    }
}
