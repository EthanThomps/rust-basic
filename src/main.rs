pub mod database;
pub mod components;

use components::app::App;
use database::driver::db_driver;


fn main() {
    db_driver();
    yew::Renderer::<App>::new().render();
}