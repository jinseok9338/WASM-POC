# WASM- POC

### WASM-POC is a repository created to conduct several proofs of concept (PoC).

## Background

1. There are two main approaches to building web apps using WebAssembly. One is to inject code compiled from other languages (in this case, Rust) into an existing JavaScript app using WebAssembly. The other is to write the entire code in Rust from the beginning using frameworks like Yew or Leptos. Yew, in particular, provides Class Components (Struct Component), Functional Components, use_state, and other hooks similar to React, making it easy for developers familiar with the React environment to transition (although the barrier of the Rust language still exists).
2. Some people here tried to create a Web App using Yew.
3. However, a question arose whether it would be possible to implement features that depend on JavaScript. -> For example, to implement Kakao social login on the web, you must depend on the JavaScript SDK provided by Kakao.
4. Also, although Rust has a powerful package manager called crate, a question arose whether it would be possible to use npm modules when you need to use npm packages.

## Goals

1. Can JavaScript SDK be used in Rust codebase?
2. Can npm modules be used in Rust codebase?

## Conclusion
To conclude, both are possible.

### Using JavaScript SDK in Rust Code

Let’s assume you want to log in with Kakao. Kakao allows social login using the Kakao SDK. In most cases, you access the object that enables social login by adding the src to the `<script>` tag. The idea is similar. First, you need to create a script tag that contains the Kakao SDK.

```
// Access the window object.
let window = web_sys::window().expect("no global `window` exists");

let document = window.document().expect("should have a document on window");
let body = document.body().expect("document should have a body");

// create script element
let script = document
    .create_element("script")?
    .dyn_into::<HtmlScriptElement>()?;
script.set_src("https://developers.kakao.com/sdk/js/kakao.min.js");
```

After the script is loaded, create a closure (callback) that calls Kakao.init and put it in the add_event_listener_with_callback function.

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

Put the init_kako function created in this way when the Page is first loaded. -> You can confirm that the Kakao object is initialized.
You can check the code at https://github.com/jinseok9338/WASM-POC/blob/main/frontend/src/Kakao/init.rs.

https://user-images.githubusercontent.com/27854958/225206567-3aff6ec1-4a06-4a2b-82d4-7ae9bb38c81e.mov

### Using NPM Packages in Rust Code

In this experiment, we explore the possibility of utilizing npm packages directly in WebAssembly code written in Rust.
The goal is to leverage the vast npm ecosystem within the WebAssembly environment, which could potentially enhance the functionality and efficiency of Wasm applications.
The code snippet provided demonstrates this approach:

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

In this code, we import a JavaScript function Dateformat from an npm package into our Rust code using the `#[wasm_bindgen(module = "/package.js")]` attribute. 
This function is then used in the Rust function my_format.

However, directly importing npm packages like this can lead to errors, as the browser may not be able to fetch the npm module. To overcome this, we use esbuild, a JavaScript bundler and minifier, to bundle our JavaScript code along with its dependencies into a single file that can be imported into our Rust code.
The following JavaScript code shows how esbuild is used:

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

This code tells esbuild to bundle the package.js file along with all its dependencies into a single file src/package.js.
With this approach, we can effectively utilize npm packages in our WebAssembly code, opening up new possibilities for web development with WebAssembly and Rust. This experiment underscores the versatility of WebAssembly and its compatibility with existing web technologies, reinforcing its potential as a powerful tool in web development.
![image](https://user-images.githubusercontent.com/27854958/225205474-5c192f2a-ac75-4bdf-b0e0-1ab97f570b82.png)
