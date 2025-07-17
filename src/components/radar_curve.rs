use dioxus::prelude::*;
use std::f32::consts::PI;
use crate::components::utils::polar_to_cartesian;
use crate::components::radar_graph::{RadarCurve, DataPoint};

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
    
    let points = (0..axes_count).map(|i| {
        let angle = -PI / 2.0 + i as f32 * axis_angle_step;
        let data_point = &props.curve.data_points[i];
        let point_radius = props.radius * (data_point.value / props.max_value).min(1.0).max(0.0);
        let (x, y) = polar_to_cartesian(point_radius, angle, props.center_x, props.center_y);
        format!("{},{}", x, y)
    }).collect::<Vec<String>>().join(" ");
    
    // Generate points for each data point
    let point_circles = (0..axes_count).map(|i| {
        let angle = -PI / 2.0 + i as f32 * axis_angle_step;
        let data_point = &props.curve.data_points[i];
        let point_radius = props.radius * (data_point.value / props.max_value).min(1.0).max(0.0);
        let (x, y) = polar_to_cartesian(point_radius, angle, props.center_x, props.center_y);
        
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
            polygon {
                points: "{points}",
                fill: "{props.curve.color}",
                "fill-opacity": "0.3",
                stroke: "{props.curve.color}",
                "stroke-width": "2"
            }
            // Add circles for each data point
            {point_circles}
        }
    }
}