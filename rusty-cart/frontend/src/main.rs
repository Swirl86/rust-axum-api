mod app;
mod models;
mod components;

use app::RustyCart;
use yew::Renderer;

fn main() {
    Renderer::<RustyCart>::new().render();
}
