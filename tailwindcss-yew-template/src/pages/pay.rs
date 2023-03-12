use yew::prelude::*;

use crate::Naver::init_pay::init_naver_pay;

#[function_component(Pay)]
pub fn pay() -> Html {

    use_effect_with_deps(
        move |_| {
            match init_naver_pay(){
                Ok(_) => {
                    log::info!("Naver Pay initialized");
                },
                Err(e) => {
                    log::error!("Naver Pay initialization failed: {:?}", e);
                }
            };
            || {}
        },
        ()
    );

    let handle_click = Callback::from(|event: MouseEvent| {
        // Handle button click here
        log::info!("Button clicked: {:?}", event);
    });

    html! {
        <div class="flex">
            <div class="w-1/2 p-4">
                <button class="w-full py-2 mb-2 bg-yellow-500 text-white font-bold rounded" onclick={&handle_click}>
                    { "Kakao Pay" }
                </button>
                <button class="w-full py-2 mb-2 bg-green-500 text-white font-bold rounded" onclick={&handle_click}>
                    { "Naver Pay" }
                </button>
                <button class="w-full py-2 mb-2 bg-orange-500 text-white font-bold rounded" onclick={&handle_click}>
                    { "Credit Card" }
                </button>
            </div>
            <div class="w-1/2 p-4">
                <input class="w-full py-2 px-4 mb-2 border border-gray-300 rounded" type="text" placeholder="Price (should be number)"/>
                <input class="w-full py-2 px-4 mb-2 border border-gray-300 rounded" type="text" placeholder="Product Name"/>
            </div>
        </div>
    }
}