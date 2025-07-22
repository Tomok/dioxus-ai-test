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
    let tooltip_content_clone3 = props.tooltip_content.clone();
    let tooltip_content_clone4 = props.tooltip_content.clone();

    let color_clone1 = color.clone();
    let color_clone2 = color.clone();
    let color_clone3 = color.clone();
    let color_clone4 = color.clone();

    let mut tooltip_state_clone1 = props.tooltip_state;
    let mut tooltip_state_clone2 = props.tooltip_state;
    let mut tooltip_state_clone3 = props.tooltip_state;
    let mut tooltip_state_clone4 = props.tooltip_state;

    // Click handler for the visible point
    let on_visible_point_click = move |_| {
        // Toggle pinned state on click
        let current_tooltip = tooltip_state_clone3.read().clone();
        match current_tooltip {
            Some(tooltip) => {
                // Create a new tooltip with toggled pinned state
                tooltip_state_clone3.set(Some(TooltipData {
                    curve_index: curve_idx,
                    label: tooltip_content_clone3.clone(),
                    x,
                    y,
                    color: color_clone3.clone(),
                    pinned: !tooltip.pinned,
                }));
            }
            None => {
                // Create a new pinned tooltip if none exists
                tooltip_state_clone3.set(Some(TooltipData {
                    curve_index: curve_idx,
                    label: tooltip_content_clone3.clone(),
                    x,
                    y,
                    color: color_clone3.clone(),
                    pinned: true,
                }));
            }
        }
    };

    // Click handler for the invisible point
    let on_invisible_point_click = move |_| {
        // Toggle pinned state on click
        let current_tooltip = tooltip_state_clone4.read().clone();
        match current_tooltip {
            Some(tooltip) => {
                // Create a new tooltip with toggled pinned state
                tooltip_state_clone4.set(Some(TooltipData {
                    curve_index: curve_idx,
                    label: tooltip_content_clone4.clone(),
                    x,
                    y,
                    color: color_clone4.clone(),
                    pinned: !tooltip.pinned,
                }));
            }
            None => {
                // Create a new pinned tooltip if none exists
                tooltip_state_clone4.set(Some(TooltipData {
                    curve_index: curve_idx,
                    label: tooltip_content_clone4.clone(),
                    x,
                    y,
                    color: color_clone4.clone(),
                    pinned: true,
                }));
            }
        }
    };

    rsx! {
        g {
            class: "data-point",
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "4",
                fill: "{color}",
                class: "data-point-circle",
                onmouseenter: move |_| {
                    // Only set tooltip if there isn't already a pinned one
                    if tooltip_state_clone1.read().as_ref().is_none_or(|t| !t.pinned) {
                        tooltip_state_clone1.set(Some(TooltipData {
                            curve_index: curve_idx,
                            label: tooltip_content_clone1.clone(),
                            x,
                            y,
                            color: color_clone1.clone(),
                            pinned: false,
                        }));
                    }
                },
                onmouseleave: move |_| {
                    // Only hide tooltip if it's not pinned
                    let is_pinned = tooltip_state_clone1.read().as_ref().is_some_and(|t| t.pinned);
                    if !is_pinned {
                        tooltip_state_clone1.set(None);
                    }
                },
                onclick: on_visible_point_click
            },
            // Add invisible larger circle to make hovering easier
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "10",
                fill: "transparent",
                class: "data-point-hitarea",
                onmouseenter: move |_| {
                    // Only set tooltip if there isn't already a pinned one
                    if tooltip_state_clone2.read().as_ref().is_none_or(|t| !t.pinned) {
                        tooltip_state_clone2.set(Some(TooltipData {
                            curve_index: curve_idx,
                            label: tooltip_content_clone2.clone(),
                            x,
                            y,
                            color: color_clone2.clone(),
                            pinned: false,
                        }));
                    }
                },
                onmouseleave: move |_| {
                    // Only hide tooltip if it's not pinned
                    let is_pinned = tooltip_state_clone2.read().as_ref().is_some_and(|t| t.pinned);
                    if !is_pinned {
                        tooltip_state_clone2.set(None);
                    }
                },
                onclick: on_invisible_point_click
            }
        }
    }
}
