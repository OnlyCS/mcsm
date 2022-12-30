use dioxus::prelude::*;

use crate::{components::tooltip::{Tooltip, TooltipLocations}, types::sizes::DefaultSizes};

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            margin: "1rem 0rem 0rem 1rem",
            display: "flex",

            Tooltip {
                text: "Home",
                location: TooltipLocations::Right,
				size: DefaultSizes::LG

                h1 { "MCSM" }
            }
        }
    })
}
