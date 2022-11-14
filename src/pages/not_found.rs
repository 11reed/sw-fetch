use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html! {
        <div class={classes!("not-found")}>
            <h1>{ "404 Error - Please return Home." }</h1>
        </div>
    }
}