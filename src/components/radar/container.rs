use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use thiserror::Error;

/// Submodules containing components directly used by the container
pub mod graph;
pub mod legend;

use graph::radar::{RadarCurve, RadarGraph};
use legend::RadarLegend;

/// Error types for RadarContainer
#[derive(Error, Debug, Clone)]
#[allow(dead_code)]
pub enum RadarError {
    #[error("Data point count mismatch in curve '{curve_name}': expected {expected} points (to match axis count), got {actual} points")]
    DataPointCountMismatch {
        curve_name: String,
        expected: usize,
        actual: usize,
    },

    #[error("No axes provided for radar graph")]
    NoAxesProvided,

    #[error("No curves provided for radar graph")]
    NoCurvesProvided,
}

/// Props for the RadarContainer component
#[derive(Props, PartialEq, Clone)]
pub struct RadarContainerProps {
    /// List of axis names for the radar graph
    axes: Vec<String>,
    /// List of curves to be displayed
    curves: Vec<RadarCurve>,
    /// Maximum value for all axes (scale)
    #[props(default = 100.0)]
    pub max_value: f32,
    /// Width of the SVG element
    #[props(default = 600)]
    pub width: u32,
    /// Height of the SVG element
    #[props(default = 500)]
    pub height: u32,
}

#[allow(dead_code)]
impl RadarContainerProps {
    /// Create a new RadarContainerProps with validation
    pub fn new(
        axes: Vec<String>,
        curves: Vec<RadarCurve>,
        max_value: Option<f32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Self, RadarError> {
        // Validate axes and curves
        if axes.is_empty() {
            return Err(RadarError::NoAxesProvided);
        }

        if curves.is_empty() {
            return Err(RadarError::NoCurvesProvided);
        }

        // Validate that each curve has the correct number of data points
        for curve in &curves {
            let data_point_count = curve.data_points.len();
            if data_point_count != axes.len() {
                return Err(RadarError::DataPointCountMismatch {
                    curve_name: curve.name.clone(),
                    expected: axes.len(),
                    actual: data_point_count,
                });
            }
        }

        // If validation passes, create the props
        Ok(Self {
            axes,
            curves,
            max_value: max_value.unwrap_or(100.0),
            width: width.unwrap_or(600),
            height: height.unwrap_or(500),
        })
    }

    /// Set both axes and curves with validation
    pub fn set_data(
        &mut self,
        axes: Vec<String>,
        curves: Vec<RadarCurve>,
    ) -> Result<(), RadarError> {
        // Validate axes and curves
        if axes.is_empty() {
            return Err(RadarError::NoAxesProvided);
        }

        if curves.is_empty() {
            return Err(RadarError::NoCurvesProvided);
        }

        // Validate that each curve has the correct number of data points
        for curve in &curves {
            let data_point_count = curve.data_points.len();
            if data_point_count != axes.len() {
                return Err(RadarError::DataPointCountMismatch {
                    curve_name: curve.name.clone(),
                    expected: axes.len(),
                    actual: data_point_count,
                });
            }
        }

        // If validation passes, update the props
        self.axes = axes;
        self.curves = curves;
        Ok(())
    }

    /// Get a reference to the axes
    pub fn axes(&self) -> &Vec<String> {
        &self.axes
    }

    /// Get a reference to the curves
    pub fn curves(&self) -> &Vec<RadarCurve> {
        &self.curves
    }
}

/// A responsive container for the radar graph and legend
///
/// This component handles the responsive layout of the radar graph and legend,
/// placing them side by side on larger screens and stacking them on smaller screens.
/// It also manages the visibility state shared between the graph and legend.
#[component]
pub fn RadarContainer(props: RadarContainerProps) -> Element {
    // Create mutable signal from props so changes to them will trigger re-renders
    // and allow for data editing
    let mut props_signal = use_signal(|| props);

    // Initialize visibility state for all curves
    let mut visible_map = use_signal(|| {
        // Start with all curves visible (true)
        props_signal
            .read()
            .curves()
            .iter()
            .map(|curve| (curve.name.clone(), true))
            .collect::<Vec<_>>()
    });

    // Update visibility map if curves have changed
    {
        let current_curve_names: Vec<String> = props_signal
            .read()
            .curves()
            .iter()
            .map(|c| c.name.clone())
            .collect();
        let existing_names: Vec<String> = visible_map
            .read()
            .iter()
            .map(|(name, _)| name.clone())
            .collect();

        // If the curves have changed, update the visibility state
        if current_curve_names != existing_names {
            let mut new_visibility = Vec::new();

            for curve in props_signal.read().curves() {
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

    // Handle legend click to toggle visibility
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

    // Handle value changes from tooltip editing
    let handle_value_change = move |change: (usize, usize, f32)| {
        let (curve_index, point_index, new_value) = change;
        let mut current_props = props_signal.read().clone();

        // Update the data point value
        if let Some(curve) = current_props.curves.get_mut(curve_index) {
            if let Some(data_point) = curve.data_points.get_mut(point_index) {
                data_point.value = new_value;
                props_signal.set(current_props);
            }
        }
    };

    // Filter curves based on visibility for the graph
    let visible_curves = {
        let props_read = props_signal.read();
        props_read
            .curves()
            .iter()
            .filter_map(|curve| {
                let is_visible = visible_map
                    .read()
                    .iter()
                    .find(|(name, _)| name == &curve.name)
                    .map(|(_, vis)| *vis)
                    .unwrap_or(true);

                if is_visible {
                    Some(curve.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    rsx! {
        div {
            // Responsive container that changes from column on small screens to row on medium and larger screens
            class: "flex flex-col md:flex-row justify-center items-center md:items-start gap-4",

            // Graph container
            div {
                class: "flex-shrink-0",
                RadarGraph {
                    axes: props_signal.read().axes().clone(),
                    curves: visible_curves,
                    max_value: props_signal.read().max_value,
                    width: props_signal.read().width,
                    height: props_signal.read().height,
                    on_value_change: handle_value_change,
                }
            }

            // Legend container - positioned below on mobile, to the right on desktop
            div {
                class: "p-4 flex-shrink-0",
                div {
                    class: "bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md",
                    h3 {
                        class: "text-lg font-semibold mb-2",
                        "Legend"
                    }
                    div {
                        class: "space-y-2",
                        RadarLegend {
                            curves: props_signal.read().curves().clone(),
                            visible_map: visible_map,
                            on_click: on_legend_click,
                            // Always use vertical layout for the legend
                            layout: "vertical".to_string(),
                        }
                    }
                }
            }
        }
    }
}
