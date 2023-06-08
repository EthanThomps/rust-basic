use yew::prelude::*;

use crate::components::todos::Docker;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <Docker/>
            <Docker/>
        </div>
    }
}