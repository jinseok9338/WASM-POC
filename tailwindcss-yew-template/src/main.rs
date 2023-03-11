extern crate reqwest_wasm;

use crate::{Kakao::login::kakao_login, Naver::login::naver_login};
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use Kakao::init::init_kakao;
use Naver::init::init_naver;

mod Kakao;
mod Naver;
mod data;

enum Msg {}

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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // set onclick to call login_request()
        let kakao_login = Callback::from(move |_| {
            log::info!("Login request");
            wasm_bindgen_futures::spawn_local(async move {
                //get kakao object from window
                let window = web_sys::window().expect("no global `window` exists");
                let kakao = window
                    .get("Kakao")
                    .expect("Kakao object not found")
                    .dyn_into::<JsValue>()
                    .expect("Failed to cast Kakao object to Object");
                log::info!("Kakao object: {:?}", kakao);
                let res = kakao_login(&kakao);
                log::info!("Login request response: {:?}", res);
                // Use res here
            });
        });

        let naver_login = Callback::from(move |_| {
            log::info!("Login request");
            wasm_bindgen_futures::spawn_local(async move {
                //get kakao object from window
                let window = web_sys::window().expect("no global `window` exists");
                let naver = window
                    .get("naver")
                    .expect("naver object not found")
                    .dyn_into::<JsValue>()
                    .expect("Failed to cast naver object to Object");
                log::info!("naver object: {:?}", naver);
                let res = naver_login(&naver);
                log::info!("Login request response: {:?}", res);
                // Use res here
            });
        });

        html! {
            <div class="flex">

            <div class="w-1/2 p-4 flex justify-center items-center">
              <div class="flex flex-row space-x-2">
                <div class="flex flex-row items-center">
                  <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e3/KakaoTalk_logo.svg/240px-KakaoTalk_logo.svg.png" class="w-6 h-6 mr-2"  />
                  <button class="bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-2 px-4 rounded" onclick={kakao_login}>
                    {"Kakao"}
                  </button>
                </div>
                <div class="flex flex-row items-center">
                  <img src="https://assets.stickpng.com/images/623afb9327d4946aceae2faf.png" class="w-6 h-6 mr-2" />
                  <a id="naverIdLogin_loginButton" href="javascript:void(0)">
                    <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded" onclick = {naver_login} id="naverIdLogin">
                   {"Naver"}
                  </button>
                    </a>
                </div>
                <div class="flex flex-row items-center">
                  <img src="https://th.bing.com/th/id/OIP.avtaP0CZ0oou1eN_so-_AQAAAA?pid=ImgDet&rs=1" class="w-6 h-6 mr-2" />
                  <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">
                    {"Google"}
                  </button>
                </div>
              </div>
            </div>

            <div class="w-1/2 p-4 flex flex-col justify-center items-center">
              <div class="w-32 h-32 rounded-full overflow-hidden">
                <img class="w-full h-full object-cover" src="https://picsum.photos/200" />
              </div>
              <h2 class="text-lg font-medium mt-4">{"John Doe"}</h2>
            </div>
          </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            match init_kakao() {
                Ok(_) => log::info!("Kakao init success"),
                Err(_) => log::info!("Kakao init failed"),
            }
            match init_naver() {
                Ok(_) => log::info!("Naver init success"),
                Err(_) => log::info!("Naver init failed"),
            }
        }
    }
}

impl App {}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
