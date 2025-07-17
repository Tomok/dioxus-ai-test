use dioxus::prelude::*;

/// Props for the Tooltip component
#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    /// The x-coordinate for the tooltip
    pub x: f32,
    /// The y-coordinate for the tooltip
    pub y: f32,
    /// Content to display in the tooltip
    pub content: String,
    /// Whether the tooltip is visible
    #[props(default = false)]
    pub visible: bool,
}

/// A simple tooltip component for displaying information on hover
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    // Position the tooltip slightly above the point
    let tooltip_x = props.x;
    let tooltip_y = props.y - 15.0;

    rsx! {
        g {
            class: "tooltip",
            rect {
                x: "{tooltip_x - 60.0}",
                y: "{tooltip_y - 25.0}",
                width: "120",
                height: "20",
                rx: "3",
                ry: "3",
                fill: "#333",
                opacity: "0.8",
            }
            text {
                x: "{tooltip_x}",
                y: "{tooltip_y - 12.0}",
                fill: "white",
                "text-anchor": "middle",
                "font-size": "12",
                "{props.content}"
            }
        }
    }
}
