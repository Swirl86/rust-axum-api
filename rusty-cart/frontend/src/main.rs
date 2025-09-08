mod app;
mod models;
mod components;
mod utils;

use app::RustyCart;
use yew::Renderer;

fn main() {
    Renderer::<RustyCart>::new().render();
}
