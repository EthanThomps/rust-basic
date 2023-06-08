pub mod database;

use database::sqrl::db_driver;
use yew::prelude::*;

#[function_component]
fn App() -> Html {

    html! {
        <div>
            <p class={classes!("docker")}>{"Docker"}</p>
        </div>
    }
}

fn main() {
    db_driver();
    yew::Renderer::<App>::new().render();
}