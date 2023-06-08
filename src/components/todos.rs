use yew::prelude::*;

#[function_component]
pub fn Docker() -> Html {
    html! {
        <div>
            <p class={classes!("docker")}>{"Docker Example"}</p>
        </div>
    }
}