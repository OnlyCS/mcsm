use dioxus::prelude::*;

use crate::components::sidebar::Sidebar;

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            id: "main",

            Sidebar {}
        }
    })
}
