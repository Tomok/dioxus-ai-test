use dioxus::prelude::*;
use crate::components::{RadarCurve, RadarGraph};
use crate::components::radar_legend::RadarLegend;
use dioxus::hooks::use_signal;

/// Props for the RadarContainer component
#[derive(Props, PartialEq, Clone)]
pub struct RadarContainerProps {
    /// List of axis names for the radar graph
    pub axes: Vec<String>,
    /// List of curves to be displayed
    pub curves: Vec<RadarCurve>,
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

/// A responsive container for the radar graph and legend
/// 
/// This component handles the responsive layout of the radar graph and legend,
/// placing them side by side on larger screens and stacking them on smaller screens.
/// It also manages the visibility state shared between the graph and legend.
#[component]
pub fn RadarContainer(props: RadarContainerProps) -> Element {
    // Initialize visibility state for all curves
    let mut visible_map = use_signal(|| {
        // Start with all curves visible (true)
        props
            .curves
            .iter()
            .map(|curve| (curve.name.clone(), true))
            .collect::<Vec<_>>()
    });

    // Update visibility map if curves have changed
    {
        let current_curve_names: Vec<String> = props.curves.iter().map(|c| c.name.clone()).collect();
        let existing_names: Vec<String> = visible_map
            .read()
            .iter()
            .map(|(name, _)| name.clone())
            .collect();

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

    // Filter curves based on visibility for the graph
    let visible_curves = props.curves.iter().filter_map(|curve| {
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
    }).collect::<Vec<_>>();

    rsx! {
        div {
            // Responsive container that changes from column on small screens to row on medium and larger screens
            class: "flex flex-col md:flex-row justify-center items-center md:items-start gap-4",
            
            // Graph container
            div {
                class: "flex-shrink-0",
                RadarGraph {
                    axes: props.axes.clone(),
                    curves: visible_curves,
                    max_value: props.max_value,
                    width: props.width,
                    height: props.height,
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
                            curves: props.curves.clone(),
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