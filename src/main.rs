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
            class: "py-16 flex h-32 w-full absolute top-0 border bg-gray-100",
            div{ class: "m-auto","ヘッダーです" }
        }
    })
}