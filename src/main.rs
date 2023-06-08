pub mod database;
pub mod components;

fn main() {
    // ::new().render()
    yew::Renderer::<components::app::App>::new().render();
}