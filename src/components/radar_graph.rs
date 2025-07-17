use dioxus::prelude::*;
use std::f32::consts::PI;

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

/// Converts polar coordinates to Cartesian coordinates
fn polar_to_cartesian(radius: f32, angle_in_radians: f32, center_x: f32, center_y: f32) -> (f32, f32) {
    let x = center_x + radius * angle_in_radians.cos();
    let y = center_y + radius * angle_in_radians.sin();
    (x, y)
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
    
    let center_x = props.width as f32 / 2.0;
    let center_y = props.height as f32 / 2.0;
    let radius = f32::min(center_x, center_y) * 0.8;
    let axis_angle_step = 2.0 * PI / axes_count as f32;
    
    // Generate axis lines and labels
    let axis_lines = (0..axes_count).map(|i| {
        let angle = -PI / 2.0 + i as f32 * axis_angle_step;
        let (end_x, end_y) = polar_to_cartesian(radius, angle, center_x, center_y);
        
        // Calculate label position (slightly beyond the end of the axis)
        let label_radius = radius * 1.1;
        let (label_x, label_y) = polar_to_cartesian(label_radius, angle, center_x, center_y);
        
        // Text anchor based on position
        let text_anchor = if label_x < center_x - 5.0 {
            "end"
        } else if label_x > center_x + 5.0 {
            "start"
        } else {
            "middle"
        };
        
        // Text vertical alignment based on position
        let dy = if label_y < center_y - 5.0 {
            "-0.5em"
        } else if label_y > center_y + 5.0 {
            "1em"
        } else {
            "0.3em"
        };
        
        rsx! {
            line {
                x1: "{center_x}",
                y1: "{center_y}",
                x2: "{end_x}",
                y2: "{end_y}",
                stroke: "#888",
                "stroke-width": "1"
            }
            text {
                x: "{label_x}",
                y: "{label_y}",
                "text-anchor": "{text_anchor}",
                dy: "{dy}",
                "font-size": "12px",
                fill: "#333",
                "{props.axes[i]}"
            }
        }
    });
    
    // Generate radar grid (concentric circles)
    let grid_levels = 5;
    let grid_circles = (1..=grid_levels).map(|level| {
        let level_radius = radius * level as f32 / grid_levels as f32;
        let level_value = props.max_value * level as f32 / grid_levels as f32;
        
        rsx! {
            circle {
                cx: "{center_x}",
                cy: "{center_y}",
                r: "{level_radius}",
                fill: "none",
                stroke: "#ddd",
                "stroke-width": "1"
            }
            text {
                x: "{center_x}",
                y: "{center_y - level_radius}",
                "text-anchor": "middle",
                "font-size": "10px",
                fill: "#999",
                "{level_value}"
            }
        }
    });
    
    // Generate curves for each data set
    let curve_polygons = props.curves.iter().map(|curve| {
        // Check if the curve has the correct number of data points
        if curve.data_points.len() != axes_count {
            return rsx! {
                g {
                    title { "Error: Curve '{curve.name}' has incorrect number of data points" }
                }
            };
        }
        
        let points = (0..axes_count).map(|i| {
            let angle = -PI / 2.0 + i as f32 * axis_angle_step;
            let data_point = &curve.data_points[i];
            let point_radius = radius * (data_point.value / props.max_value).min(1.0).max(0.0);
            let (x, y) = polar_to_cartesian(point_radius, angle, center_x, center_y);
            format!("{},{}", x, y)
        }).collect::<Vec<String>>().join(" ");
        
        rsx! {
            g {
                polygon {
                    points: "{points}",
                    fill: "{curve.color}",
                    "fill-opacity": "0.3",
                    stroke: "{curve.color}",
                    "stroke-width": "2"
                }
                // Add points for each data point
                {(0..axes_count).map(|i| {
                    let angle = -PI / 2.0 + i as f32 * axis_angle_step;
                    let data_point = &curve.data_points[i];
                    let point_radius = radius * (data_point.value / props.max_value).min(1.0).max(0.0);
                    let (x, y) = polar_to_cartesian(point_radius, angle, center_x, center_y);
                    
                    rsx! {
                        circle {
                            cx: "{x}",
                            cy: "{y}",
                            r: "4",
                            fill: "{curve.color}"
                        }
                    }
                })}
            }
        }
    });
    
    // Generate legend items
    let legend_items = props.curves.iter().enumerate().map(|(index, curve)| {
        rsx! {
            g {
                transform: "translate(10, {10 + index as u32 * 20})",
                rect {
                    width: "15",
                    height: "15",
                    fill: "{curve.color}"
                }
                text {
                    x: "20",
                    y: "12",
                    "font-size": "12px",
                    "{curve.name}"
                }
            }
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
                {grid_circles}
                
                // Axis lines and labels
                {axis_lines}
                
                // Curve polygons
                {curve_polygons}
                
                // Legend
                g {
                    class: "legend",
                    transform: "translate({props.width - 120}, 20)",
                    {legend_items}
                }
            }
        }
    }
}

// Basic tests for the RadarGraph component
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_polar_to_cartesian() {
        // Test basic conversions
        let center_x = 100.0;
        let center_y = 100.0;
        
        // Right direction (0 degrees)
        let (x, y) = polar_to_cartesian(10.0, 0.0, center_x, center_y);
        assert_eq!(x as i32, 110);
        assert_eq!(y as i32, 100);
        
        // Up direction (270 degrees or -90 degrees)
        let (x, y) = polar_to_cartesian(10.0, -PI/2.0, center_x, center_y);
        assert_eq!(x as i32, 100);
        assert_eq!(y as i32, 90);
        
        // Left direction (180 degrees)
        let (x, y) = polar_to_cartesian(10.0, PI, center_x, center_y);
        assert_eq!(x as i32, 90);
        assert_eq!(y as i32, 100);
        
        // Down direction (90 degrees)
        let (x, y) = polar_to_cartesian(10.0, PI/2.0, center_x, center_y);
        assert_eq!(x as i32, 100);
        assert_eq!(y as i32, 110);
    }
}