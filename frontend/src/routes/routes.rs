use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{login::Login, pay::Pay, npm_code::NpmCode};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/pay")]
    Pay,
    #[at("/npm")]
    Npm,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => {
            html! { <Login /> }
        }
        Route::Npm => {
            html! { <NpmCode/> }
        }
        Route::NotFound => {
            html! { <h1>{"404"}</h1> }
        }
        Route::Pay => {
            html! { <Pay/> }
        },
    }
}