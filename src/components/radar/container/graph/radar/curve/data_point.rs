use crate::components::radar::container::graph::radar::TooltipData;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DataPointProps {
    /// X coordinate of the data point
    pub x: f32,
    /// Y coordinate of the data point
    pub y: f32,
    /// Index of the curve this point belongs to
    pub curve_index: usize,
    /// Color of the curve/point
    pub color: String,
    /// Content to display in the tooltip
    pub tooltip_content: String,
    /// Shared tooltip state
    pub tooltip_state: Signal<Option<TooltipData>>,
}

/// Component for rendering a data point in a radar curve
#[component]
pub fn DataPoint(props: DataPointProps) -> Element {
    let curve_idx = props.curve_index;
    let color = props.color.clone();
    let x = props.x;
    let y = props.y;

    // Create closures that clone the tooltip content when needed
    let tooltip_content_clone1 = props.tooltip_content.clone();
    let tooltip_content_clone2 = props.tooltip_content.clone();
    let color_clone1 = color.clone();
    let color_clone2 = color.clone();

    let mut tooltip_state_clone1 = props.tooltip_state;
    let mut tooltip_state_clone2 = props.tooltip_state;

    rsx! {
        g {
            class: "data-point",
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "4",
                fill: "{color}",
                // Create a slightly larger transparent circle for better hover target
                onmouseenter: move |_| {
                    tooltip_state_clone1.set(Some(TooltipData {
                        curve_index: curve_idx,
                        label: tooltip_content_clone1.clone(),
                        x,
                        y,
                        color: color_clone1.clone(),
                    }));
                },
                onmouseleave: move |_| {
                    tooltip_state_clone1.set(None);
                }
            },
            // Add invisible larger circle to make hovering easier
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "10",
                fill: "transparent",
                onmouseenter: move |_| {
                    tooltip_state_clone2.set(Some(TooltipData {
                        curve_index: curve_idx,
                        label: tooltip_content_clone2.clone(),
                        x,
                        y,
                        color: color_clone2.clone(),
                    }));
                },
                onmouseleave: move |_| {
                    tooltip_state_clone2.set(None);
                }
            }
        }
    }
}
