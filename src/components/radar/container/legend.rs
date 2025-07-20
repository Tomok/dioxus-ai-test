use super::graph::radar::RadarCurve;
use dioxus::prelude::*;

/// Props for the RadarLegend component
#[derive(Props, PartialEq, Clone)]
pub struct RadarLegendProps {
    /// List of curves to display in the legend
    pub curves: Vec<RadarCurve>,
    /// Optional visibility map for curves
    #[props(optional)]
    pub visible_map: Option<Signal<Vec<(String, bool)>>>,
    /// Optional click handler for legend items
    #[props(optional)]
    pub on_click: Option<EventHandler<String>>,
    /// Optional layout direction (horizontal or vertical)
    #[props(default = "vertical".to_string())]
    pub layout: String,
}

/// A standalone legend component for the radar graph
///
/// This component can be positioned independently of the radar graph
/// and can be displayed in either horizontal or vertical layout.
#[component]
pub fn RadarLegend(props: RadarLegendProps) -> Element {
    // Create legend items with click handling
    let legend_items = props.curves.iter().enumerate().map(|(index, curve)| {
        // Check if the curve is visible
        let is_visible = match &props.visible_map {
            Some(vis_map) => vis_map
                .read()
                .iter()
                .find(|(name, _)| name == &curve.name)
                .map(|(_, vis)| *vis)
                .unwrap_or(true),
            None => true, // Default to visible if no visibility state is provided
        };

        // Set up click handler
        let curve_name = curve.name.clone();
        let on_click = props.on_click;
        let onclick = move |_| {
            if let Some(handler) = &on_click {
                handler.call(curve_name.clone());
            }
        };

        // Determine position based on layout
        let transform = if props.layout == "horizontal" {
            format!("translate({}, 0)", index as u32 * 120)
        } else {
            format!("translate(0, {})", index as u32 * 20)
        };

        // Determine text classes based on visibility
        let text_classes = if is_visible {
            "text-xs cursor-pointer text-text dark:text-white"
        } else {
            "text-xs cursor-pointer text-text dark:text-white line-through opacity-70"
        };

        // Determine rectangle classes based on visibility
        let rect_classes = if is_visible {
            "cursor-pointer"
        } else {
            "cursor-pointer opacity-30"
        };

        rsx! {
            g {
                transform: "{transform}",
                onclick: onclick,
                rect {
                    width: "15",
                    height: "15",
                    fill: "{curve.color}",
                    class: "{rect_classes}"
                }
                text {
                    x: "20",
                    y: "12",
                    class: "{text_classes}",
                    "{curve.name}"
                }
            }
        }
    });

    rsx! {
        div {
            class: "legend-container",
            svg {
                // Adjust width based on layout
                width: if props.layout == "horizontal" { "100%" } else { "150px" },
                // Adjust height based on layout and number of items
                height: if props.layout == "horizontal" { "30px" } else { format!("{}px", props.curves.len() * 20 + 10) },
                g {
                    class: "legend",
                    {legend_items}
                }
            }
        }
    }
}
