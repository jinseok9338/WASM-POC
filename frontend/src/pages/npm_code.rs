use js_sys::Date;
use yew::prelude::*;

use crate::pages::date_fns::my_format;

#[function_component(NpmCode)]
pub fn npm_code() -> Html {
    let date = use_state(|| Option::<String>::None);
    
    let onclick = {
        let date = date.clone();
        Callback::from(move |_| {
            let now = Date::new_0();
            let formatted_date = my_format(&now, "'Today is a' eeee");
            date.set(Some(formatted_date));
        })
    };

    html! {
        <div class="flex flex-col items-center">
            <p class="text-lg font-bold mb-2">{"This time is generated with date-fns npm package"}</p>
            <p class="text-gray-500 mb-4">{date.clone().as_ref().map(|d| d.as_str()).unwrap_or("null")}</p>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" {onclick}>
                {"Generate Date"}
            </button>
        </div>
    }
}