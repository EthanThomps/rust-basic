pub mod database;
pub mod components;

fn main() {
    yew::Renderer::<components::app::App>::new().render();
}