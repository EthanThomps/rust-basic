use yew::prelude::*;

#[function_component]
pub fn Docker() -> Html {
    html! {
        <div>
            <p class={classes!("docker")}>{"Docker Example"}</p>
        </div>
    }
}

#[function_component]
pub fn Todos() -> Html {
    html! {
        <div class={classes!()}>
            
            <h1>{"Todos!"}</h1>
            
        </div>
    }
}

