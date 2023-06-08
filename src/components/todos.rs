use yew::prelude::*;
// if styles don't work then re install tailwindcss intellisense

#[function_component]
pub fn AppTitle() -> Html {
    html! {
        <div>
            <p class={classes!("mainTitle")}>{"Todo App"}</p>
        </div>
    }
}

#[function_component]
pub fn Todos() -> Html {
    html! {
        <div class={classes!("todoCardHolder")}>

            <TodoCard/>

        </div>
    }
}

#[function_component]
pub fn TodoCard() -> Html {
    html! {
        <ul class={classes!("todoCard")}>
            <li class={classes!("todoCardItemHolder")}>
                <h1 class={classes!("todoCardItem")}>{"Task"}</h1>
                <p class={classes!("todoCardMadeItem")}>{"Unkown"}</p>
            </li>
            <li class={classes!("todoCardItemHolder")}>
                <h1 class={classes!("todoCardItem")}>{"Username"}</h1>
                <p class={classes!("todoCardMadeItem")}>{"Unkown"}</p>
            </li>
            <li class={classes!("todoCardItemHolder")}>
                <h1 class={classes!("todoCardItem")}>{"Tags"}</h1>
                <p class={classes!("todoCardMadeItem")}>{"Unkown"}</p>
            </li>            
        </ul>
    }
}

#[function_component]
pub fn TodoCardMaker() -> Html {
    html! {
        <form action="/temp" class={classes!("formCardHolder")}>
            <div class={classes!("todoCard")}>
                <div class={classes!("formItemHolder")}>
                    <label class={classes!("todoCardItem")} for="task">{"Task"}</label>
                    <div>
                        <input type="text" name="task" id="task" class={classes!("formItem")}/>
                    </div>
                </div>
                <div class={classes!("formItemHolder")}>
                    <label class={classes!("todoCardItem")} for="username">{"Username"}</label>
                    <div>
                        <input type="text" name="username" id="username" class={classes!("formItem")}/>
                    </div>
                </div>
                <div class={classes!("formItemHolder")}>
                    <label class={classes!("todoCardItem")} for="tag">{"Tag"}</label>
                    <div>
                        <input type="text" name="tag" id="tag" class={classes!("formItem")}/> 
                    </div>
                </div>            
                <div class={classes!("formBtnHolder")}>
                    <button type="submit" class={classes!("todoCardButton")}>{"Enter"}</button>
                    <button type="reset" class={classes!("todoCardButton")}>{"Reset"}</button>
                </div>      
            </div>
        </form>
    }
}