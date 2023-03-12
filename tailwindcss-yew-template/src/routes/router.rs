use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::routes::routes::{Route, switch};

pub struct Router {}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }



    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {

            <BrowserRouter>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        }
    }
}