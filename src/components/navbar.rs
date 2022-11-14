use yew::prelude::*;
use yew_router::prelude::Link;

use crate::routes::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class={classes!("nav-bar")}>
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                    <a class="nav-link"><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></a>
                </li>
                <li class="nav-item">
                    <a class="nav-link"><Link<Route> to={Route::About}>{ "About" }</Link<Route>></a>
                </li>
            </ul>
        </div>
    }
}