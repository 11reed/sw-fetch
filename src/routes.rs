use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::home::HomePage;
use crate::pages::about::AboutPage;
use crate::pages::not_found::NotFoundPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}