use crate::components::radar_graph::RadarCurve;
use crate::components::utils::polar_to_cartesian;
use dioxus::prelude::*;
use std::f32::consts::PI;

#[derive(Props, PartialEq, Clone)]
pub struct RadarCurveVisualProps {
    /// Data for the curve
    pub curve: RadarCurve,
    /// Labels for each axis
    pub axes: Vec<String>,
    /// Center X coordinate
    pub center_x: f32,
    /// Center Y coordinate
    pub center_y: f32,
    /// Radius of the graph
    pub radius: f32,
    /// Maximum value on the scale
    pub max_value: f32,
}

/// Component for rendering a single curve in a radar graph
#[component]
pub fn RadarCurveVisual(props: RadarCurveVisualProps) -> Element {
    let axes_count = props.axes.len();

    // Check if the curve has the correct number of data points
    if props.curve.data_points.len() != axes_count {
        return rsx! {
            g {
                title { "Error: Curve '{props.curve.name}' has incorrect number of data points" }
            }
        };
    }

    let axis_angle_step = 2.0 * PI / axes_count as f32;

    // Calculate all points first for easier processing
    let points = (0..axes_count)
        .map(|i| {
            let angle = -PI / 2.0 + i as f32 * axis_angle_step;
            let data_point = &props.curve.data_points[i];
            let point_radius =
                props.radius * (data_point.value / props.max_value).clamp(0.0, 1.0);
            let (x, y) = polar_to_cartesian(point_radius, angle, props.center_x, props.center_y);
            (x, y)
        })
        .collect::<Vec<(f32, f32)>>();

    // Generate SVG path with curved lines
    let mut path_data = String::new();

    // Start with a move to the first point
    path_data.push_str(&format!("M{},{} ", points[0].0, points[0].1));

    // Create curved lines between points
    for i in 0..axes_count {
        let next_idx = (i + 1) % axes_count;
        let current = points[i];
        let next = points[next_idx];

        // Calculate control points for the curve
        // Use a factor of the distance between points for the control point distance
        let curve_factor = 0.3;


        // Calculate tangent points
        let tangent_angle1 = (-PI / 2.0 + i as f32 * axis_angle_step) % (2.0 * PI) + PI / 2.0;
        let tangent_angle2 =
            (-PI / 2.0 + next_idx as f32 * axis_angle_step) % (2.0 * PI) - PI / 2.0;

        // Distance between points (chord length)
        let dist = ((next.0 - current.0).powi(2) + (next.1 - current.1).powi(2)).sqrt();

        // Control points
        let control_dist = dist * curve_factor;
        let (cp1_x, cp1_y) = (
            current.0 + control_dist * tangent_angle1.cos(),
            current.1 + control_dist * tangent_angle1.sin(),
        );
        let (cp2_x, cp2_y) = (
            next.0 + control_dist * tangent_angle2.cos(),
            next.1 + control_dist * tangent_angle2.sin(),
        );

        // Add cubic Bezier curve
        path_data.push_str(&format!(
            "C {},{} {},{} {},{} ",
            cp1_x, cp1_y, cp2_x, cp2_y, next.0, next.1
        ));
    }

    // Close the path
    path_data.push('Z');

    // Generate points for each data point
    let point_circles = (0..axes_count).map(|i| {
        let (x, y) = points[i];

        rsx! {
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "4",
                fill: "{props.curve.color}"
            }
        }
    });

    rsx! {
        g {
            class: "radar-curve",
            path {
                d: "{path_data}",
                fill: "{props.curve.color}",
                "fill-opacity": "0.3",
                stroke: "{props.curve.color}",
                "stroke-width": "2",
                "stroke-linejoin": "round"
            }
            // Add circles for each data point
            {point_circles}
        }
    }
}
