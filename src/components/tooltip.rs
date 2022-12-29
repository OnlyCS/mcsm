use dioxus::prelude::*;

pub enum TooltipLocations {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Props)]
pub struct TooltipProps<'a> {
    text: &'a str,
    children: Element<'a>,

	#[props(default=TooltipLocations::Top)]
    location: TooltipLocations,
}

pub fn Tooltip<'a>(cx: Scope<'a, TooltipProps<'a>>) -> Element {
	// set variable location to &cx.props.location as &str lowercase
	let location = match &cx.props.location {
		TooltipLocations::Top => "top",
		TooltipLocations::Left => "left",
		TooltipLocations::Right => "right",
		TooltipLocations::Bottom => "bottom",
	};

    cx.render(rsx! {
        div {
            class: "tooltip {location}",
            "tooltip-text": "{cx.props.text}",

            &cx.props.children
        }
    })
}
