# WASM- POC
`WASM-POC` 는 몇가지 개념 증명(Proof of Concept)을 하고 싶어서 만든 레포(Repository)이다. 

## 배경 
1. Web Assembly 를 이용하여 웹 앱을 만드는 방식은 크게 2가지로 분류 된다. 기존의 javascript 로 쓰여진 앱에 웹 어셈블리로 컴파일 된 타 언어 코드( 여기서는 Rust 이다) 를 주입하는 방식, 혹은 처음부터 Yew 혹은 Leptos 를 이용하여 전체 코드를 러스트로 작성 하는 방식이 있다. 
Yew 는 특히 React 와 같은 Class Component( Struct Component), Functional Component, use_state 와 같은 hook 등을 제공 하기 때문에 React 환경에 익숙한 개발자면 쉽게 넘어 올 수 가 있다. (물론 Rust 언어의 장벽은 그대로 존재 한다.)
2. 혹자는 여기서 Yew 를 이용하여 Web App 을 만들려고 하였다.
3. 그러나 Javascript 에 의존성이 있는 기능을 구현할 수 있을까 하는 의문이 생겼다. -> 예를 들어 Kakao 소셜 로그인은 웹에서 구현 하려면 Kakao 가 제공하는 Javascript SDK 에 의존 하는 방법 밖에 없다. 
4. 또한 Rust 에서도 강력한 crate 라는 패키지 매니저가 있지만 과연 npm package 를 써야 하는 경우 npm 모듈을 사용할 수 있을까라는 의문이 생겼다.

## 목표
1. Javascript SDK 를 Rust 코드 베이스에서 쓸 수 있을까?
2. Npm 모듈을 Rust 코드 베이스에서 쓸 수 있을까?

## 결론 
결론부터 말하자면 두가지 다 가능 하다.

### Javascript SDK 를 Rust 코드에서 사용하기
카카오 로그인 을 하고 싶다고 가정해 보자. 카카오는 카카오 SDK 를 이용한 소셜 로그인이 가능하다. 보통의 경우에는 `<script> tag` 에 src 를 추가하여 소셜 로그인 을 가능 하게 하는 객체에 접근한다.
아이디어는 비슷하다. 일단 카카오 SDK 를 넣는 script tag 를 만들어야 한다.
```
// window 객체에 접근한다.
let window = web_sys::window().expect("no global `window` exists");

let document = window.document().expect("should have a document on window");
let body = document.body().expect("document should have a body");

// create script element
let script = document
    .create_element("script")?
    .dyn_into::<HtmlScriptElement>()?;
script.set_src("https://developers.kakao.com/sdk/js/kakao.min.js");
```

이후 스크립트가 로드 되면 Kakao.init 을 호출하는 closure (callback) 을 만들어서 add_event_listener_with_callback 함수에 넣어준다
```
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
```

그렇게 만들어진 init_kako 함수를 Page 가 처음 로드 될때 넣어준다. -> Kakao 객체가 init 된것을 확인 할 수 있다.

https://github.com/jinseok9338/WASM-POC/blob/main/frontend/src/Kakao/init.rs 에서 코드를 확인 할 수 있다.

https://user-images.githubusercontent.com/27854958/225206567-3aff6ec1-4a06-4a2b-82d4-7ae9bb38c81e.mov

### Npm Package 이용하기
다음과 같은 함수를 Rust code 에서 사용하고 싶다고 가정해 보자
```
// package.js

import { format } from 'date-fns';

export function Dateformat(date, formatString) {
    return format(date, formatString);
}
```

위의 자바 스크립트 코드는 date-fns 모듈에 의존적이다.

그래서 다음과 같이 가져오면 브라우저에서는 date-fns 모듈을 가지고 오지 못해서 에러를 반환한다. (package.js 는 가져오지만 date-fns 는 가져오지 못한다)
```
use js_sys::Date;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/package.js")]
extern "C" {
    fn Dateformat(date: &Date, formatString: &str) -> String;
}

#[wasm_bindgen]
pub fn my_format(date: &Date, format_string: &str) -> String {
    Dateformat(date, format_string)
}
```
그러나 생각해 보면 자바스크립트만 가지고 오면 되는것이다. 그래서 요즘 web-framwork 의 방식을 빌리기로 하였다
자바스크립트를 webpack 을 이용하여 번들링 해주기로 하였다. 여기서는 esbuld 를 사용하기로 하자

1. esbuild 를 설치한다. `npm run esbuild`
2. 프로젝트 루트 디렉토리에 `exbuild.js` 파일을 만든다. 
```
// esbuild
esbuild.build({
  entryPoints: ['package.js'],
  bundle: true,
  outfile: 'src/package.js',
  format: 'esm',
  minify: true,
}).catch(() => process.exit(1));
```

3. package.json 에 `"esbuild":"node esbuild.js"` 명령어를 추가한다.
4. src/package.js 를 보면 빌드된 자바 스크립트가 보일 것이다.
5. 빌드된 자바 스크립트를 가져와서 쓴다.
```
use js_sys::Date;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/package.js")]
extern "C" {
    fn Dateformat(date: &Date, formatString: &str) -> String;
}

#[wasm_bindgen]
pub fn my_format(date: &Date, format_string: &str) -> String {
    Dateformat(date, format_string)
}
```
정상적으로 작동 하는것을 확인 할 수 있다. ![image](https://user-images.githubusercontent.com/27854958/225205474-5c192f2a-ac75-4bdf-b0e0-1ab97f570b82.png)
예시는 https://github.com/jinseok9338/WASM-POC/blob/main/frontend/src/pages/date_fns.rs 에서 확인 가능하다.




