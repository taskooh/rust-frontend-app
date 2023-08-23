#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    log::debug!("App");
    cx.render(rsx! {
        div {
            "hello world"
        }
    })
}