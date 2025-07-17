use dioxus::prelude::*;
use crate::components::radar_graph::RadarCurve;

#[derive(Props, PartialEq, Clone)]
pub struct RadarLegendProps {
    /// List of curves to display in the legend
    pub curves: Vec<RadarCurve>,
    /// X position of the legend
    pub x: f32,
    /// Y position of the legend
    pub y: f32,
}

/// Component for rendering the legend of a radar graph
#[component]
pub fn RadarLegend(props: RadarLegendProps) -> Element {
    let legend_items = props.curves.iter().enumerate().map(|(index, curve)| {
        rsx! {
            g {
                transform: "translate(0, {index as u32 * 20})",
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
        g {
            class: "legend",
            transform: "translate({props.x}, {props.y})",
            {legend_items}
        }
    }
}