#![allow(non_snake_case)]

use dioxus::prelude::*;
use pages::index::Index;

pub mod components;
pub mod pages;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/", Index {} }
        }
    })
}
