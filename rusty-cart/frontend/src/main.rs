mod app;
mod models;

use app::RustyCart;
use yew::Renderer;

fn main() {
    Renderer::<RustyCart>::new().render();
}
