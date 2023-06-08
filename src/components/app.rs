use yew::prelude::*;

use crate::components::todos::*;
use crate::database::driver::db_driver;
#[function_component]
pub fn App() -> Html {
    db_driver();

    html! {
        <div>
            <Docker/>
            <Todos/>
        </div>
    }
}