
use wasm_bindgen::{JsValue, JsCast};
use web_sys::MouseEvent;
use yew::Callback;





pub fn naver_login(_naver: &JsValue)  {
    log::info!("login with naver");
}

pub fn naver_login_callback() -> Callback<MouseEvent> {
    Callback::from(move |_event:MouseEvent| {
        log::info!("Login request");
        wasm_bindgen_futures::spawn_local(async move {
            //get naver object from window
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
    })
}
