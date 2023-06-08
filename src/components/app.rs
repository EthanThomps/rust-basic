use yew::prelude::*;

use crate::components::todos::Docker;
use crate::database::driver::db_driver;
#[function_component]
pub fn App() -> Html {
    db_driver();

    html! {
        <div>
            <Docker/>
            <Docker/>
        </div>
    }
}