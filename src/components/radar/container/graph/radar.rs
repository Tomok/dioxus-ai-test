use crate::components::editable_tooltip::EditableTooltip;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;

// Import components
use self::axis::RadarAxis;
use self::curve::RadarCurveVisual;
use self::grid::RadarGrid;

// Component modules
pub mod axis;
pub mod curve;
pub mod grid;

/// Data structure for tooltip information
#[derive(Clone, PartialEq)]
pub struct TooltipData {
    /// Index of the curve
    pub curve_index: usize,
    /// Index of the data point within the curve
    pub point_index: usize,
    /// Label to display in the tooltip
    pub label: String,
    /// Current value of the data point
    pub value: f32,
    /// X-coordinate of the tooltip
    pub x: f32,
    /// Y-coordinate of the tooltip
    pub y: f32,
    /// Color of the curve
    pub color: String,
    /// Whether the tooltip is pinned (remains visible when not hovering)
    pub pinned: bool,
    /// Whether the tooltip is currently being edited
    pub editing: bool,
}

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
#[derive(Props, Clone)]
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
    /// Callback for when a data point value is updated
    #[props(optional)]
    pub on_value_change: Option<EventHandler<(usize, usize, f32)>>,
}

// Implement PartialEq manually since EventHandler doesn't implement PartialEq
impl PartialEq for RadarGraphProps {
    fn eq(&self, other: &Self) -> bool {
        self.axes == other.axes
            && self.curves == other.curves
            && self.max_value == other.max_value
            && self.width == other.width
            && self.height == other.height
        // Note: We can't compare EventHandler, so we ignore on_value_change in equality
    }
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

    // Create a shared signal for tooltip state that all curves can access
    let mut tooltip_state = use_signal(|| None::<TooltipData>);

    // Generate curves for each data set
    let curve_components = props.curves.iter().enumerate().map(|(curve_idx, curve)| {
        rsx! {
            RadarCurveVisual {
                curve: curve.clone(),
                curve_index: curve_idx,
                axes: props.axes.clone(),
                center_x: center_x,
                center_y: center_y,
                radius: radius,
                max_value: props.max_value,
                tooltip_state: tooltip_state,
            }
        }
    });

    // Handle tooltip editing events
    let mut tooltip_state_for_handlers = tooltip_state;

    let handle_start_edit = move |_| {
        let current = tooltip_state_for_handlers.read().clone();
        if let Some(mut data) = current {
            data.editing = true;
            tooltip_state_for_handlers.set(Some(data));
        }
    };

    let handle_complete_edit = {
        let on_value_change = props.on_value_change;
        let mut tooltip_state_for_complete = tooltip_state;

        move |new_value: f32| {
            let current = tooltip_state_for_complete.read().clone();
            if let Some(mut data) = current {
                // Call the value change callback if provided
                if let Some(callback) = &on_value_change {
                    callback.call((data.curve_index, data.point_index, new_value));
                }

                // Update the tooltip data and stop editing
                data.value = new_value;
                data.editing = false;
                data.label = format!(
                    "{}: {}",
                    data.label.split(':').next().unwrap_or(""),
                    new_value
                );
                tooltip_state_for_complete.set(Some(data));
            }
        }
    };

    let handle_cancel_edit = move |_| {
        let current = tooltip_state.read().clone();
        if let Some(mut data) = current {
            data.editing = false;
            tooltip_state.set(Some(data));
        }
    };

    // Generate editable tooltip component based on shared state
    let tooltip = {
        let tooltip_info = tooltip_state.read();

        if let Some(data) = &*tooltip_info {
            rsx! {
                EditableTooltip {
                    x: data.x,
                    y: data.y,
                    content: data.label.clone(),
                    value: data.value,
                    curve_index: data.curve_index,
                    point_index: data.point_index,
                    visible: true,
                    pinned: data.pinned,
                    editing: data.editing,
                    color: data.color.clone(),
                    on_start_edit: handle_start_edit,
                    on_complete_edit: handle_complete_edit,
                    on_cancel_edit: handle_cancel_edit,
                }
            }
        } else {
            rsx! {}
        }
    };

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

                // Tooltip at the very top layer
                {tooltip}
            }
        }
    }
}
