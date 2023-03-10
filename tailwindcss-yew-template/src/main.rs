extern crate reqwest_wasm;

use yew::prelude::*;
use dotenv::dotenv;
use std::env;
mod data;
use serde_json::{json, to_string};

enum Msg {
    FetchNewUser,
    UpdatePerson(data::Result),
    LoginRequest
}

struct App {
    user: data::Result,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user: data::Result::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::FetchNewUser => {
                link.send_future(App::get_person());

                false
            }

            Msg::UpdatePerson(person) => {
                log::info!("Update Person: {:?}", { &person });
                self.user = person;

                true
            }
            
            Msg::LoginRequest => {
                
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // set onclick to call login_request()
        let onclick = Callback::from(move |_| {
            log::info!("Login request");
            wasm_bindgen_futures::spawn_local(async move {
                let res = App::login_request().await;
                log::info!("Login request response: {:?}", res);
                // Use res here
            });
        });
       

        html! {
            <div class="h-screen bg-gray-600 w-full flex flex-col items-center justify-center gap-y-4">
                <div class="w-96 h-80 bg-gray-800 shadow-md border-indigo-400 h-auto w-auto p-4 pl-8 pr-8 rounded-md font-medium text-xl flex flex-col items-center">
                    <img class="rounded-full w-24 h-24" src={self.user.picture.large.to_string()} />
                    <div class="mt-4 mb-4 flex flex-col gap-y-1">
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Gender: "}</span>
                            <span class="text-gray-300 font-light ml-2">{&self.user.gender}</span>
                        </div>
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Registered: "}</span>
                            <span class="text-gray-300 font-light ml-2">{&self.user.registered.date}</span>
                        </div>
                        <div class="flex flex-row">
                            <span class="text-gray-100 font-medium">{"Phone number: "}</span>
                            <span class="text-gray-300 font-light ml-2">{&self.user.phone}</span>
                        </div>
                    </div>
                    <span class="font-bold text-xl text-center text-indigo-400"> {&self.user.name.first} {" "} {&self.user.name.last} </span>
                    <span class="font-light text-lg text-center text-gray-400"> {"Password: "} {&self.user.login.password} </span>
                </div>

                <button {onclick} class="bg-indigo-400 shadow-md h-auto w-auto pl-4 pr-4 pb-2 pt-2 rounded-md font-medium text-xl text-white hover:bg-indigo-300">{"Find date"}</button>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(App::get_person());
        }
    }
}

impl App {
    async fn get_person() -> Msg {
        let res = reqwest_wasm::get("https://randomuser.me/api/1.2")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let parsed_json = serde_json::from_str::<data::Root>(res.as_str()).unwrap();
        return Msg::UpdatePerson((*parsed_json.results.first().unwrap()).clone());
    }
    async fn login_request() -> String   {
        dotenv().ok();
        
        let server_url: &str = env!("SERVER_URL");
     
        let res = reqwest_wasm::Client::new()
        .get(format!("{}/login", server_url))
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
       return res;

       
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
