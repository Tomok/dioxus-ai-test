use crate::components::utils::polar_to_cartesian;
use dioxus::prelude::*;
use std::f32::consts::PI;

#[derive(Props, PartialEq, Clone)]
pub struct RadarAxisProps {
    /// Labels for each axis
    pub labels: Vec<String>,
    /// Center X coordinate
    pub center_x: f32,
    /// Center Y coordinate
    pub center_y: f32,
    /// Radius of the graph
    pub radius: f32,
}

/// Component for rendering the axes of a radar graph
#[component]
pub fn RadarAxis(props: RadarAxisProps) -> Element {
    let axes_count = props.labels.len();
    let axis_angle_step = 2.0 * PI / axes_count as f32;

    // Generate axis lines and labels
    let axis_lines = (0..axes_count).map(|i| {
        let angle = -PI / 2.0 + i as f32 * axis_angle_step;
        let (end_x, end_y) =
            polar_to_cartesian(props.radius, angle, props.center_x, props.center_y);

        // Calculate label position (slightly beyond the end of the axis)
        let label_radius = props.radius * 1.1;
        let (label_x, label_y) =
            polar_to_cartesian(label_radius, angle, props.center_x, props.center_y);

        // Text anchor based on position
        let text_anchor = if label_x < props.center_x - 5.0 {
            "end"
        } else if label_x > props.center_x + 5.0 {
            "start"
        } else {
            "middle"
        };

        // Text vertical alignment based on position
        let dy = if label_y < props.center_y - 5.0 {
            "-0.5em"
        } else if label_y > props.center_y + 5.0 {
            "1em"
        } else {
            "0.3em"
        };

        rsx! {
            line {
                x1: "{props.center_x}",
                y1: "{props.center_y}",
                x2: "{end_x}",
                y2: "{end_y}",
                stroke: "#888888",
                class: "dark:stroke-gray-500",
                "stroke-width": "1"
            }
            text {
                x: "{label_x}",
                y: "{label_y}",
                "text-anchor": "{text_anchor}",
                dy: "{dy}",
                "font-size": "12px",
                fill: "#333333",
                class: "dark:fill-gray-200",
                "{props.labels[i]}"
            }
        }
    });

    rsx! {
        g { class: "radar-axes", {axis_lines} }
    }
}
