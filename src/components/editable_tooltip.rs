use dioxus::prelude::*;

/// Props for the EditableTooltip component
#[derive(Props, PartialEq, Clone)]
pub struct EditableTooltipProps {
    /// The x-coordinate for the tooltip
    pub x: f32,
    /// The y-coordinate for the tooltip
    pub y: f32,
    /// Content to display in the tooltip
    pub content: String,
    /// Current value being edited
    pub value: f32,
    /// Index of the curve (used to detect data point changes)
    #[props(default = 0)]
    pub curve_index: usize,
    /// Index of the data point within the curve (used to detect data point changes)
    #[props(default = 0)]
    pub point_index: usize,
    /// Whether the tooltip is visible
    #[props(default = false)]
    pub visible: bool,
    /// Whether the tooltip is pinned (stays visible when not hovering)
    #[props(default = false)]
    pub pinned: bool,
    /// Whether the tooltip is in editing mode
    #[props(default = false)]
    pub editing: bool,
    /// Color of the associated curve
    pub color: String,
    /// Callback when editing starts
    pub on_start_edit: EventHandler<()>,
    /// Callback when editing is completed with new value
    pub on_complete_edit: EventHandler<f32>,
    /// Callback when editing is cancelled
    pub on_cancel_edit: EventHandler<()>,
}

/// An editable tooltip component that allows inline editing of values when pinned
#[component]
pub fn EditableTooltip(props: EditableTooltipProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    // Position the tooltip slightly above the point
    let tooltip_x = props.x;
    let tooltip_y = props.y - 15.0;

    // State for the current input value - only used during active editing
    let mut input_value = use_signal(|| props.value.to_string());

    // Create a unique key for the current data point to detect switches
    let current_data_point_key = format!("{}_{}", props.curve_index, props.point_index);
    let mut last_data_point_key = use_signal(|| current_data_point_key.clone());

    // Reset input value when switching to a different data point
    if current_data_point_key != *last_data_point_key.read() {
        input_value.set(props.value.to_string());
        last_data_point_key.set(current_data_point_key);
    }

    // Add class for pinned state
    let pinned_class = if props.pinned { "tooltip-pinned" } else { "" };

    // Handle input changes
    let handle_input = {
        let mut input_value = input_value;
        move |evt: FormEvent| {
            input_value.set(evt.value());
        }
    };

    // Handle key press events
    let handle_key_press = {
        let mut input_value = input_value;
        let on_complete_edit = props.on_complete_edit;
        let on_cancel_edit = props.on_cancel_edit;
        let original_value = props.value;

        move |evt: KeyboardEvent| {
            match evt.key() {
                Key::Enter => {
                    // Try to parse the input value
                    let input_str = input_value.read().clone();
                    if let Ok(new_value) = input_str.parse::<f32>() {
                        on_complete_edit.call(new_value);
                    } else {
                        // Reset to original value if parsing fails
                        input_value.set(original_value.to_string());
                        on_cancel_edit.call(());
                    }
                }
                Key::Escape => {
                    // Cancel editing and reset to original value
                    input_value.set(original_value.to_string());
                    on_cancel_edit.call(());
                }
                _ => {}
            }
        }
    };

    // Handle blur event to complete editing
    let handle_blur = {
        let mut input_value = input_value;
        let on_complete_edit = props.on_complete_edit;
        let on_cancel_edit = props.on_cancel_edit;
        let original_value = props.value;

        move |_evt: FocusEvent| {
            // Try to parse the input value
            let input_str = input_value.read().clone();
            if let Ok(new_value) = input_str.parse::<f32>() {
                on_complete_edit.call(new_value);
            } else {
                // Reset to original value if parsing fails
                input_value.set(original_value.to_string());
                on_cancel_edit.call(());
            }
        }
    };

    // Parse the content to separate name and value
    let (name, _) = if let Some(colon_pos) = props.content.find(':') {
        let name = props.content[..colon_pos].trim();
        let value_part = props.content[colon_pos + 1..].trim();
        (name, value_part)
    } else {
        (props.content.as_str(), "")
    };

    rsx! {
        g {
            class: "editable-tooltip {pinned_class}",

            // Tooltip background
            rect {
                x: "{tooltip_x - 60.0}",
                y: "{tooltip_y - 25.0}",
                width: "120",
                height: "20",
                rx: "3",
                ry: "3",
                fill: "#333333",
                class: "dark:fill-gray-800",
                opacity: "0.8",
            }

            // Single foreignObject container for all tooltip content
            foreignObject {
                x: "{tooltip_x - 60.0}",
                y: "{tooltip_y - 25.0}",
                width: "120",
                height: "20",

                div {
                    class: "w-full h-full flex items-center justify-center px-2 text-white text-xs",

                    if props.editing && props.pinned {
                        // Editing mode: name label + input field
                        div {
                            class: "flex items-center justify-between w-full gap-1",

                            // Name label
                            span {
                                class: "text-white flex-shrink-0",
                                style: "font-size: 10px;",
                                "{name}:"
                            }

                            // Input field
                            input {
                                r#type: "number",
                                step: "0.1",
                                value: "{input_value.read()}",
                                class: "flex-1 min-w-0 px-1 text-center text-white border border-gray-500 rounded outline-none focus:border-blue-400 bg-white bg-opacity-10",
                                style: "font-size: 10px; -moz-appearance: textfield;",
                                oninput: handle_input,
                                onkeydown: handle_key_press,
                                onblur: handle_blur,
                                // Auto-focus when editing starts
                                autofocus: true,
                            }
                        }
                    } else {
                        // Normal mode: display full content
                        span {
                            class: "text-white cursor-pointer text-center",
                            style: "font-size: 12px;",
                            "{props.content}"
                        }
                    }
                }
            }

            // Visual indicator for pinned tooltips
            if props.pinned && !props.editing {
                circle {
                    cx: "{tooltip_x + 45.0}",
                    cy: "{tooltip_y - 20.0}",
                    r: "2",
                    fill: "{props.color}",
                    class: "tooltip-pin-indicator"
                }
            }
        }
    }
}
