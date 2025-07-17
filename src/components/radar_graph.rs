use dioxus::prelude::*;
use dioxus::hooks::use_signal;

/// A data point for the radar graph.
/// Each data point represents a value for a specific axis.
#[derive(Clone, PartialEq)]
pub struct DataPoint {
    pub value: f32,
    pub label: String,
}

/// Data for a single curve in the radar graph.
#[derive(Clone, PartialEq)]
pub struct RadarCurve {
    pub name: String,
    pub data_points: Vec<DataPoint>,
    pub color: String,
}

/// Props for the RadarGraph component.
#[derive(Props, PartialEq, Clone)]
pub struct RadarGraphProps {
    /// List of axis names for the radar graph
    pub axes: Vec<String>,
    /// List of curves to be displayed
    pub curves: Vec<RadarCurve>,
    /// Maximum value for all axes (scale)
    #[props(default = 100.0)]
    pub max_value: f32,
    /// Width of the SVG element
    #[props(default = 500)]
    pub width: u32,
    /// Height of the SVG element
    #[props(default = 500)]
    pub height: u32,
}

/// RadarGraph component that displays data in a radar chart
///
/// # Props
/// - axes: List of axis labels
/// - curves: List of RadarCurve objects to display
/// - max_value: Maximum value for scaling the axes
/// - width: Width of the SVG
/// - height: Height of the SVG
#[component]
pub fn RadarGraph(props: RadarGraphProps) -> Element {
    let axes_count = props.axes.len();

    // Return early if there are no axes
    if axes_count == 0 {
        return rsx! {
            div { "No axes provided for radar graph" }
        };
    }

    // Initialize visibility state for all curves
    let mut visible_map = use_signal(|| {
        // Start with all curves visible (true)
        props.curves
            .iter()
            .map(|curve| (curve.name.clone(), true))
            .collect::<Vec<_>>()
    });

    // Update visibility map if curves have changed
    {
        let current_curve_names: Vec<String> = props.curves.iter().map(|c| c.name.clone()).collect();
        let existing_names: Vec<String> = visible_map.read().iter().map(|(name, _)| name.clone()).collect();
        
        // If the curves have changed, update the visibility state
        if current_curve_names != existing_names {
            let mut new_visibility = Vec::new();
            
            for curve in &props.curves {
                // Try to find existing visibility setting
                let is_visible = visible_map
                    .read()
                    .iter()
                    .find(|(name, _)| name == &curve.name)
                    .map(|(_, vis)| *vis)
                    .unwrap_or(true); // Default to visible for new curves
                
                new_visibility.push((curve.name.clone(), is_visible));
            }
            
            visible_map.set(new_visibility);
        }
    }

    let center_x = props.width as f32 / 2.0;
    let center_y = props.height as f32 / 2.0;
    let radius = f32::min(center_x, center_y) * 0.8;

    // Import required components
    use crate::components::{
        radar_axis::RadarAxis, radar_curve::RadarCurveVisual, radar_grid::RadarGrid,
        radar_legend::RadarLegend,
    };

    // Handle legend click
    let on_legend_click = move |name: String| {
        let current_map = visible_map.read().clone();
        let mut new_map = Vec::new();
        
        for (curve_name, is_visible) in current_map {
            if curve_name == name {
                new_map.push((curve_name, !is_visible)); // Toggle this curve
            } else {
                new_map.push((curve_name, is_visible)); // Keep state for others
            }
        }
        
        visible_map.set(new_map);
    };

    // Generate curves for each data set, respecting visibility
    let curve_components = props.curves.iter().filter_map(|curve| {
        // Find visibility status for this curve
        let vis_map = visible_map.read();
        let is_visible = vis_map
            .iter()
            .find(|(name, _)| name == &curve.name)
            .map(|(_, vis)| *vis)
            .unwrap_or(true); // Default to visible
        
        // Only render if visible
        if is_visible {
            Some(rsx! {
                RadarCurveVisual {
                    curve: curve.clone(),
                    axes: props.axes.clone(),
                    center_x: center_x,
                    center_y: center_y,
                    radius: radius,
                    max_value: props.max_value,
                }
            })
        } else {
            None
        }
    });

    rsx! {
        div {
            class: "radar-graph-container",
            svg {
                width: "{props.width}",
                height: "{props.height}",
                view_box: "0 0 {props.width} {props.height}",

                // Grid circles
                RadarGrid {
                    center_x: center_x,
                    center_y: center_y,
                    radius: radius,
                    max_value: props.max_value,
                }

                // Axis lines and labels
                RadarAxis {
                    labels: props.axes.clone(),
                    center_x: center_x,
                    center_y: center_y,
                    radius: radius,
                }

                // Curve polygons
                {curve_components}

                // Legend
                RadarLegend {
                    curves: props.curves.clone(),
                    x: props.width as f32 - 120.0,
                    y: 20.0,
                    visible_map: visible_map,
                    on_click: on_legend_click,
                }
            }
        }
    }
}
