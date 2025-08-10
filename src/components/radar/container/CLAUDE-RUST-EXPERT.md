# CLAUDE-RUST-EXPERT.md - Radar Container Architecture

## Container Component Deep Dive

### Advanced State Architecture
The radar container demonstrates sophisticated state management patterns for complex interactive components.

#### 1. Multi-Dimensional State Design
**State Separation by Concern:**
```rust
// Data state (what to display)
let mut props_signal = use_signal(|| props);

// UI state (how to display)  
let mut visible_map = use_signal(|| /* visibility state */);

// Interaction state (user interactions)
let mut tooltip_state = use_signal(|| None::<TooltipData>);
```

#### 2. State Synchronization Patterns
**Reactive State Dependencies:**
```rust
// When curves change, update visibility map
if current_curve_names != existing_names {
    let mut new_visibility = Vec::new();
    for curve in props_signal.read().curves() {
        // Preserve existing state, default new items
        let is_visible = existing_state.get(&curve.name).unwrap_or(true);
        new_visibility.push((curve.name.clone(), is_visible));
    }
    visible_map.set(new_visibility);
}
```

### Complex Event Handling Architecture

#### 1. Multi-Source Event Coordination
**Event Handler Composition:**
```rust
// Legend click handler
let on_legend_click = move |name: String| {
    // Immutable update pattern
    let current_map = visible_map.read().clone();
    let new_map = current_map.into_iter()
        .map(|(curve_name, is_visible)| {
            if curve_name == name {
                (curve_name, !is_visible) // Toggle
            } else {
                (curve_name, is_visible)  // Preserve
            }
        })
        .collect();
    visible_map.set(new_map);
};

// Value change handler with props mutation
let handle_value_change = move |change: (usize, usize, f32)| {
    let (curve_index, point_index, new_value) = change;
    let mut current_props = props_signal.read().clone();
    
    // Deep mutation of nested data structure
    if let Some(curve) = current_props.curves.get_mut(curve_index) {
        if let Some(data_point) = curve.data_points.get_mut(point_index) {
            data_point.value = new_value;
            props_signal.set(current_props); // Trigger re-render
        }
    }
};
```

#### 2. Type-Safe Event Data
**Structured Event Parameters:**
```rust
// Type alias for clarity
type ValueChangeEvent = (usize, usize, f32);
//                      ^curve ^point ^new_value

// Usage preserves type safety
#[props(optional)]
pub on_value_change: Option<EventHandler<ValueChangeEvent>>,
```

### Advanced Props Patterns

#### 1. Builder Pattern Implementation
**Comprehensive Validation:**
```rust
impl RadarContainerProps {
    pub fn new(
        axes: Vec<String>,
        curves: Vec<RadarCurve>, 
        max_value: Option<f32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Self, RadarError> {
        // Multi-step validation
        Self::validate_axes(&axes)?;
        Self::validate_curves(&curves)?;
        Self::validate_curve_data_consistency(&axes, &curves)?;
        
        Ok(Self {
            axes,
            curves, 
            max_value: max_value.unwrap_or(100.0),
            width: width.unwrap_or(600),
            height: height.unwrap_or(500),
        })
    }
    
    fn validate_curve_data_consistency(
        axes: &[String], 
        curves: &[RadarCurve]
    ) -> Result<(), RadarError> {
        for curve in curves {
            if curve.data_points.len() != axes.len() {
                return Err(RadarError::DataPointCountMismatch {
                    curve_name: curve.name.clone(),
                    expected: axes.len(),
                    actual: curve.data_points.len(),
                });
            }
        }
        Ok(())
    }
}
```

#### 2. Immutable Update Patterns
**Clone-and-Modify Strategy:**
```rust
// Read current state
let mut current_props = props_signal.read().clone();

// Modify cloned state
current_props.curves[curve_index].data_points[point_index].value = new_value;

// Set new state (triggers re-render)
props_signal.set(current_props);
```

### Graph & Legend Coordination

#### 1. Shared State Management
**Bidirectional Data Flow:**
```rust
// Graph filters curves based on visibility
let visible_curves = props_signal.read()
    .curves()
    .iter()
    .filter(|curve| is_curve_visible(&curve.name, &visible_map))
    .cloned()
    .collect::<Vec<_>>();

// Legend controls visibility state
RadarLegend {
    curves: props_signal.read().curves().clone(),
    visible_map: visible_map,           // Shared signal
    on_click: on_legend_click,          // Visibility toggle
    layout: "vertical".to_string(),
}
```

#### 2. Component Communication Patterns
**Props-Based Coordination:**
- Graph receives filtered data (read-only)
- Legend receives full data + visibility signal (read-write)
- Container orchestrates all interactions

### Error Recovery & Resilience

#### 1. Graceful Degradation
```rust
// Handle empty state gracefully
if props_signal.read().axes().is_empty() {
    return rsx! {
        div { 
            class: "error-state",
            "No data available for radar graph" 
        }
    };
}
```

#### 2. State Consistency Maintenance
```rust
// Ensure visibility map stays in sync with curves
// Handle curve additions/removals gracefully
// Preserve user preferences when possible
```

### Performance Optimization Strategies

#### 1. Signal Access Optimization
```rust
// Minimize signal reads
let props_read = props_signal.read();  // Single read
let axes = props_read.axes().clone();   // Use cached read
let curves = props_read.curves().clone(); // Reuse read
```

#### 2. Computed Property Caching
```rust
// Expensive filtering computed once per render
let visible_curves = {
    // Complex computation here
    // Result cached by signal system
};
```

#### 3. Conditional Rendering Optimization  
```rust
// Only render legend if curves exist
if !props_signal.read().curves().is_empty() {
    rsx! { RadarLegend { /* ... */ } }
} else {
    rsx! {}
}
```

### Responsive Design Integration

#### 1. Layout State Management
```rust
// Responsive container classes
class: "flex flex-col md:flex-row justify-center items-center md:items-start gap-4"

// Component positioning based on screen size
// Graph: Fixed size, centered
// Legend: Responsive placement (below on mobile, side on desktop)
```

#### 2. Component Sizing Strategy
```rust
// Graph container: Fixed size for consistent visualization
div { class: "flex-shrink-0" }

// Legend container: Flexible with proper spacing  
div { class: "p-4 flex-shrink-0" }
```