use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::MouseEvent;
use yew::Callback;

pub fn kakao_login(kakao: &JsValue) {
    let auth = js_sys::Reflect::get(kakao, &"Auth".into()).unwrap();
    let login = js_sys::Reflect::get(&auth.as_ref().into(), &"login".into()).unwrap();
    let login_function = login.dyn_into::<js_sys::Function>().unwrap();

    let api = js_sys::Reflect::get(kakao, &"API".into()).unwrap();
    let api = Rc::new(RefCell::new(api));
    let request = js_sys::Reflect::get(&api.borrow().as_ref().into(), &"request".into()).unwrap();
    let request_function = request.dyn_into::<js_sys::Function>().unwrap();

    let success_callback = {
        let api = Rc::<_>::clone(&api);

        Closure::wrap(Box::new(move |_response: JsValue| {
            let success_callback = Closure::wrap(Box::new(move |response: JsValue| {
                log::info!("Kakao.API.request succeeded {:?}", response);
            }) as Box<dyn FnMut(_)>);

            let fail_callback = Closure::wrap(Box::new(move |_error: JsValue| {
                log::warn!("Kakao.API.request failed");
            }) as Box<dyn FnMut(_)>);

            let options = js_sys::Object::new();
            js_sys::Reflect::set(
                &options.clone().into(),
                &"url".into(),
                &"/v2/user/me".into(),
            )
            .unwrap();
            js_sys::Reflect::set(
                &options.clone().into(),
                &"success".into(),
                success_callback.as_ref().unchecked_ref(),
            )
            .unwrap();
            js_sys::Reflect::set(
                &options.clone().into(),
                &"fail".into(),
                fail_callback.as_ref().unchecked_ref(),
            )
            .unwrap();

            request_function
                .call1(&api.borrow().as_ref().into(), &options.into())
                .unwrap();

            success_callback.forget();
            fail_callback.forget();
        }) as Box<dyn FnMut(_)>)
    };

    let fail_callback = Closure::<_>::wrap(Box::<_>::new(move |_error: JsValue| {
        log::warn!("Kakao.Auth.login failed");
    }) as Box<dyn FnMut(_)>);

    let options = js_sys::Object::new();
    js_sys::Reflect::set(
        &options.clone().into(),
        &"success".into(),
        success_callback.as_ref().unchecked_ref(),
    )
    .unwrap();
    js_sys::Reflect::set(
        &options.clone().into(),
        &"fail".into(),
        fail_callback.as_ref().unchecked_ref(),
    )
    .unwrap();

    login_function.call1(&auth.into(), &options.into()).unwrap();
    success_callback.forget();
    fail_callback.forget();
}

pub fn kakao_login_callback() -> Callback<MouseEvent> {
    Callback::from(move |_event:MouseEvent| {
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
    })
}
