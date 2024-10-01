use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::common::{Home, PageNotFound};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route)  -> Html {
    match routes {
        Route::Home => {
            html! {
                <Home />
            }
        },
        _ => {
            html! {
                <PageNotFound />
            }
        }
    }
}
