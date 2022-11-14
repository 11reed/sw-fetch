use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod components;
mod routes;

use crate::routes::Route;
use crate::routes::switch;
use crate::components::navbar::Navbar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
