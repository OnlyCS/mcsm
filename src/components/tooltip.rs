use dioxus::prelude::*;

use crate::types::sizes::DefaultSizes;

pub enum TooltipLocations {
    Top,
    Left,
    Right,
    Bottom
}

#[derive(Props)]
pub struct TooltipProps<'a> {
    text: &'a str,
    children: Element<'a>,

	#[props(default=TooltipLocations::Top)]
    location: TooltipLocations,

	#[props(default=DefaultSizes::MD)]
	size: DefaultSizes
}

pub fn Tooltip<'a>(cx: Scope<'a, TooltipProps<'a>>) -> Element {
	// set variable location to &cx.props.location as &str lowercase
	let location = match &cx.props.location {
		TooltipLocations::Top => "top",
		TooltipLocations::Left => "left",
		TooltipLocations::Right => "right",
		TooltipLocations::Bottom => "bottom"
	};

	let size = match &cx.props.size {
		DefaultSizes::XS => "xs",
		DefaultSizes::SM => "sm",
		DefaultSizes::MD => "md",
		DefaultSizes::LG => "lg",
		DefaultSizes::XL => "xl"
	};

    cx.render(rsx! {
        div {
            class: "tooltip {location} {size}",
            "tooltip-text": "{cx.props.text}",

            &cx.props.children
        }
    })
}
