# WASM- POC


WebAssembly (Wasm) has opened up new possibilities for web development, offering near-native performance for web applications.
However, one of the challenges developers face when working with Wasm is the integration of existing JavaScript libraries and npm packages.


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
