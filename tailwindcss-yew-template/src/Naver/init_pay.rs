use gloo_utils::format::JsValueSerdeExt;
use serde_json::to_value;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::HtmlScriptElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PayOptions {
    mode: String,
    clientId: String,
}


pub fn init_naver_pay()-> Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    
    // create script element
    let script = document
        .create_element("script")?
        .dyn_into::<HtmlScriptElement>()?;
    script.set_src("https://nsp.pay.naver.com/sdk/js/naverpay.min.js");
    
    // Make closure to init the naver pay sdk with options
    let closure = Closure::wrap(Box::new(move || {
        let o_pay: JsValue = js_sys::Reflect::get(&window, &JsValue::from_str("Naver"))
            .unwrap()
            .into();
        let o_pay: JsValue = js_sys::Reflect::get(&o_pay, &JsValue::from_str("Pay"))
            .unwrap()
            .into();
        let o_pay  = js_sys::Reflect::get(&o_pay, &JsValue::from_str("create"))
            .unwrap()
            .dyn_into::<js_sys::Function>().unwrap();
        
        let options = PayOptions {
            mode: "development".to_string(),
            clientId: env!("NAVER_CLIENT_ID").to_string(),
        };
        let options_value = to_value(options).unwrap();
        // cast Value into JsValue
        let options_value = JsValue::from_serde(&options_value).unwrap();
        // call o_pay with options as an argment
        o_pay.call1(&JsValue::NULL, &options_value);

    }) as Box<dyn FnMut()>);
    script.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())?;
    closure.forget(); // memory leak
    body.append_child(&script)?;
    
    Ok(())
  
}




