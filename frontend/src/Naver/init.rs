use js_sys::Reflect;
use serde_json::to_value;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::HtmlScriptElement;
use serde::{Deserialize, Serialize};
use gloo_utils::format::JsValueSerdeExt;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginOptions {
    clientId: String,
    callbackUrl: String,
    isPopup: bool,
    callbackHandle: bool,
}


pub fn init_naver() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // create script element
    let script = document
        .create_element("script")?
        .dyn_into::<HtmlScriptElement>()?;
    script.set_src("https://static.nid.naver.com/js/naveridlogin_js_sdk_2.0.2.js");

    let closure = Closure::wrap(Box::new(|| {
        // define Kakao object here
        let global = js_sys::global();
        let naver =
            Reflect::get(&global, &JsValue::from_str("naver")).expect("naver object not found");
        let login_with_naver_id =
        Reflect::get(&naver, &JsValue::from_str("LoginWithNaverId")).expect("LoginWithNaverId class not found").dyn_into::<js_sys::Function>()
        .expect("Failed to cast LoginWithNaverId to Function");
        
        
        let options = LoginOptions {
        clientId: env!("NAVER_CLIENT_ID").to_string(),
        callbackUrl: "http://127.0.0.1:3000".to_string(),
            isPopup: false,
            callbackHandle: true,
        
        };
        
        let options_value = to_value(options).unwrap();
        // cast Value into JsValue
        let options_value = JsValue::from_serde(&options_value).unwrap();
        
        let login_with_naver_id_instance = js_sys::Reflect::construct(
            &login_with_naver_id,
            &js_sys::Array::of1(&options_value),
        )
        .expect("Failed to construct LoginWithNaverId instance");
        
        
        //call init method inside LoginWithNaverId instance
        let init_method =
            Reflect::get(&login_with_naver_id_instance, &JsValue::from_str("init")).expect("init method not found").dyn_into::<js_sys::Function>().expect("Failed to cast init method to Function");
        
        init_method.call1(&login_with_naver_id_instance, &JsValue::NULL).expect("Failed to call init method");
    }) as Box<dyn Fn()>);
    script.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())?;
    closure.forget();

    body.append_child(&script)?;
    Ok(())
}

