#![allow(non_snake_case)]
use routes::router::Router;

extern crate reqwest_wasm;


mod Kakao;
mod Naver;
mod pages;
mod routes;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Router>::new().render();
}
