use crate::components::radar_graph::RadarCurve;
use dioxus::prelude::*;

#[component]
pub fn RadarLegend(
    curves: Vec<RadarCurve>,
    x: f32,
    y: f32,
    #[props(optional)] visible_map: Option<Signal<Vec<(String, bool)>>>,
    #[props(optional)] on_click: Option<EventHandler<String>>,
) -> Element {
    // Create legend items with click handling
    let legend_items = curves.iter().enumerate().map(|(index, curve)| {
        // Check if the curve is visible
        let is_visible = match &visible_map {
            Some(vis_map) => {
                vis_map
                    .read()
                    .iter()
                    .find(|(name, _)| name == &curve.name)
                    .map(|(_, vis)| *vis)
                    .unwrap_or(true)
            }
            None => true, // Default to visible if no visibility state is provided
        };

        // Set up click handler
        let curve_name = curve.name.clone();
        let on_click = on_click.clone();
        let onclick = move |_| {
            if let Some(handler) = &on_click {
                handler.call(curve_name.clone());
            }
        };

        // Apply styling based on visibility
        let text_style = if is_visible {
            "font-size: 12px; cursor: pointer;"
        } else {
            "font-size: 12px; cursor: pointer; text-decoration: line-through; opacity: 0.7;"
        };

        let rect_style = if is_visible {
            "cursor: pointer;"
        } else {
            "cursor: pointer; opacity: 0.3;"
        };

        rsx! {
            g {
                transform: "translate(0, {index as u32 * 20})",
                onclick: onclick,
                rect {
                    width: "15",
                    height: "15",
                    fill: "{curve.color}",
                    style: "{rect_style}"
                }
                text {
                    x: "20",
                    y: "12",
                    style: "{text_style}",
                    "{curve.name}"
                }
            }
        }
    });

    rsx! {
        g {
            class: "legend",
            transform: "translate({x}, {y})",
            {legend_items}
        }
    }
}
