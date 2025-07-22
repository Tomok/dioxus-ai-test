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
    let x = props.x;
    let y = props.y;

    // Create separate clones for each closure to avoid move errors
    let tooltip_content_click = props.tooltip_content.clone();
    let mut tooltip_state_click = props.tooltip_state;
    let color_click = props.color.clone();

    let tooltip_content_enter = props.tooltip_content.clone();
    let mut tooltip_state_enter = props.tooltip_state;
    let color_enter = props.color.clone();

    let mut tooltip_state_leave = props.tooltip_state;

    // Clone color once more for the render function
    let color_render = props.color.clone();

    // Click handler that always pins on first click, unpins on second
    let handle_click = move |_| {
        let current_tooltip = tooltip_state_click.read().clone();

        // Check if this is the same data point and it's already pinned
        let is_same_point_and_pinned = current_tooltip
            .as_ref()
            .is_some_and(|t| t.curve_index == curve_idx && t.x == x && t.y == y && t.pinned);

        if is_same_point_and_pinned {
            // If it's already pinned, unpin it
            tooltip_state_click.set(Some(TooltipData {
                curve_index: curve_idx,
                label: tooltip_content_click.clone(),
                x,
                y,
                color: color_click.clone(),
                pinned: false,
            }));
        } else {
            // Otherwise, pin it (overriding any other pinned tooltip)
            tooltip_state_click.set(Some(TooltipData {
                curve_index: curve_idx,
                label: tooltip_content_click.clone(),
                x,
                y,
                color: color_click.clone(),
                pinned: true,
            }));
        }
    };

    let handle_mouseenter = move |_| {
        // Only set tooltip if there isn't already a pinned one
        if tooltip_state_enter
            .read()
            .as_ref()
            .is_none_or(|t| !t.pinned)
        {
            tooltip_state_enter.set(Some(TooltipData {
                curve_index: curve_idx,
                label: tooltip_content_enter.clone(),
                x,
                y,
                color: color_enter.clone(),
                pinned: false,
            }));
        }
    };

    let handle_mouseleave = move |_| {
        // Only hide tooltip if it's not pinned
        let is_pinned = tooltip_state_leave
            .read()
            .as_ref()
            .is_some_and(|t| t.pinned);
        if !is_pinned {
            tooltip_state_leave.set(None);
        }
    };

    rsx! {
        g {
            class: "data-point",
            // Visible circle - just for display
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "4",
                fill: "{color_render}",
                class: "data-point-circle",
            },
            // Invisible larger circle for all interactions
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "10",
                fill: "transparent",
                class: "data-point-hitarea",
                onmouseenter: handle_mouseenter,
                onmouseleave: handle_mouseleave,
                onclick: handle_click,
            }
        }
    }
}
