use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RadarGridProps {
    /// Center X coordinate
    pub center_x: f32,
    /// Center Y coordinate
    pub center_y: f32,
    /// Radius of the graph
    pub radius: f32,
    /// Maximum value on the scale
    pub max_value: f32,
    /// Number of grid levels to display
    #[props(default = 5)]
    pub grid_levels: u32,
}

/// Component for rendering the grid circles of a radar graph
#[component]
pub fn RadarGrid(props: RadarGridProps) -> Element {
    let grid_circles = (1..=props.grid_levels).map(|level| {
        let level_radius = props.radius * level as f32 / props.grid_levels as f32;
        let level_value = props.max_value * level as f32 / props.grid_levels as f32;

        rsx! {
            circle {
                cx: "{props.center_x}",
                cy: "{props.center_y}",
                r: "{level_radius}",
                fill: "none",
                stroke: "#dddddd",
                class: "dark:stroke-gray-600",
                "stroke-width": "1"
            }
            text {
                x: "{props.center_x}",
                y: "{props.center_y - level_radius}",
                "text-anchor": "middle",
                "font-size": "10px",
                fill: "#999999",
                class: "dark:fill-gray-400",
                "{level_value}"
            }
        }
    });

    rsx! {
        g { class: "radar-grid", {grid_circles} }
    }
}
