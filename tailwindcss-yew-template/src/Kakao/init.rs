use js_sys::Reflect;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::HtmlScriptElement;

pub fn init_kakao() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // create script element
    let script = document
        .create_element("script")?
        .dyn_into::<HtmlScriptElement>()?;
    script.set_src("https://developers.kakao.com/sdk/js/kakao.min.js");
    // append script to body
    // add event listener for load event
    let closure = Closure::wrap(Box::new(|| {
        // define Kakao object here
        let global = js_sys::global();
        let kakao =
            Reflect::get(&global, &JsValue::from_str("Kakao")).expect("Kakao object not found");
        let init_method =
            Reflect::get(&kakao, &JsValue::from_str("init")).expect("init method not found");
        let init_fn = init_method
            .dyn_ref::<js_sys::Function>()
            .expect("Failed to cast init method to Function");

        let api_key = env!("KAKAO_API_KEY");
        let key = JsValue::from_str(api_key);
        init_fn
            .call1(&kakao, &key)
            .expect("Failed to call init method");
    }) as Box<dyn Fn()>);

    script.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())?;
    closure.forget();
    body.append_child(&script)?;
    Ok(())
}
