use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about() -> Html {
    html! {
        <div class={classes!("container")}>
            <div class={classes!("info")}>
                <h1>{ "About" }</h1>
            </div>
        </div>
    }
}