# CLAUDE-RUST-EXPERT.md - Radar Component Module

## Radar Graph Component Architecture

### Module Structure & Responsibilities
The radar module demonstrates advanced Rust patterns for building complex, interactive data visualizations:

- **container.rs** - Main radar container with responsive layout and state management
- **container/graph/** - SVG-based visualization components
- **container/legend.rs** - Interactive legend for curve visibility

### Advanced Rust Patterns in Radar Components

#### 1. Complex State Management
**Multi-Signal Architecture:**
```rust
// Props signal for reactive data updates
let mut props_signal = use_signal(|| props);

// Visibility state for curve toggling
let mut visible_map = use_signal(|| /* initial visibility state */);

// Shared tooltip state across components
let mut tooltip_state = use_signal(|| None::<TooltipData>);
```

#### 2. Validation-Heavy Design
**Comprehensive Input Validation:**
```rust
#[derive(Error, Debug, Clone)]
pub enum RadarError {
    #[error("Data point count mismatch in curve '{curve_name}': expected {expected} points, got {actual} points")]
    DataPointCountMismatch { curve_name: String, expected: usize, actual: usize },
    
    #[error("No axes provided for radar graph")]
    NoAxesProvided,
    
    #[error("No curves provided for radar graph")]
    NoCurvesProvided,
}
```

**Props Validation Pattern:**
- Constructor methods return `Result<Props, RadarError>`
- Runtime validation with detailed error messages
- Graceful error handling in UI components

#### 3. Signal-Based Data Synchronization

**Reactive Data Updates:**
```rust
// Update visibility map when curves change
if current_curve_names != existing_names {
    // Preserve existing visibility state for unchanged curves
    // Set default visibility for new curves
    // Remove state for deleted curves
}
```

**Event Handler Composition:**
```rust
let handle_value_change = move |change: (usize, usize, f32)| {
    let (curve_index, point_index, new_value) = change;
    // Update props signal, which triggers re-render
    // Maintain immutability while allowing controlled mutation
};
```

#### 4. Complex Props Design

**Builder Pattern for Props:**
```rust
impl RadarContainerProps {
    pub fn new(
        axes: Vec<String>,
        curves: Vec<RadarCurve>,
        max_value: Option<f32>,
        width: Option<u32>, 
        height: Option<u32>,
    ) -> Result<Self, RadarError>
    
    pub fn set_data(&mut self, axes: Vec<String>, curves: Vec<RadarCurve>) -> Result<(), RadarError>
}
```

### Data Flow Architecture

#### 1. Hierarchical State Management
```
RadarContainer (owns all state)
├── props_signal (immutable data)
├── visible_map (UI state) 
└── tooltip_state (interaction state)
    │
    ├─> RadarGraph (receives filtered data)
    │   └─> RadarCurveVisual (shared tooltip state)
    │       └─> DataPoint (tooltip mutations)
    └─> RadarLegend (visibility control)
```

#### 2. Event Propagation Patterns

**Bottom-Up Events:**
- Data point interactions → tooltip state changes
- Legend clicks → visibility state changes
- Value edits → props signal updates

**Top-Down Data Flow:**
- Props changes → filtered curve data
- Visibility changes → rendered curves
- Tooltip state → tooltip rendering

#### 3. Computed Properties Pattern
```rust
// Derived state from multiple signals
let visible_curves = {
    let props_read = props_signal.read();
    props_read.curves()
        .iter()
        .filter_map(|curve| {
            let is_visible = visible_map.read()
                .iter()
                .find(|(name, _)| name == &curve.name)
                .map(|(_, vis)| *vis)
                .unwrap_or(true);
            
            if is_visible { Some(curve.clone()) } else { None }
        })
        .collect::<Vec<_>>()
};
```

### Error Handling Strategy

#### 1. Layered Error Handling
- **Props Level**: Validation during construction
- **Component Level**: Graceful degradation for invalid state
- **UI Level**: Error messages rendered in place of components

#### 2. Error Recovery Patterns
```rust
// Early return with error display
if axes_count == 0 {
    return rsx! { div { "No axes provided for radar graph" } };
}

// Partial rendering with error indicators
if props.curve.data_points.len() != axes_count {
    return rsx! {
        g { title { "Error: Curve has incorrect number of data points" } }
    };
}
```

### Performance Optimization Techniques

#### 1. Signal Optimization
- Strategic use of `read()` vs `with()` for signal access
- Computed properties cached automatically by signals
- Minimal signal subscriptions to reduce re-renders

#### 2. Data Structure Efficiency
```rust
// Pre-compute expensive operations
let points = (0..axes_count)
    .map(|i| {
        // Complex polar coordinate calculations
        // Done once, used multiple times
    })
    .collect::<Vec<(f32, f32)>>();
```

#### 3. Render Optimization
- `PartialEq` implementations prevent unnecessary re-renders
- Conditional rendering based on visibility state
- Efficient SVG path generation

### Mathematical & Geometric Patterns

#### 1. Coordinate System Abstractions
```rust
// Polar to Cartesian conversion utility
let (x, y) = polar_to_cartesian(radius, angle, center_x, center_y);

// Angle calculation for regular polygons
let axis_angle_step = 2.0 * PI / axes_count as f32;
let angle = -PI / 2.0 + i as f32 * axis_angle_step;
```

#### 2. Curve Generation Algorithms
- Cubic Bezier curves for smooth radar polygons
- Control point calculation based on tangent angles
- Path data generation for SVG rendering

#### 3. Interactive Hit Detection
- Point-in-circle detection for data points
- Coordinate transformation for mouse events
- Tooltip positioning calculations

### Component Lifecycle Management

#### 1. Initialization Patterns
```rust
// Initialize signals with computed state
let mut visible_map = use_signal(|| {
    props_signal.read()
        .curves()
        .iter()
        .map(|curve| (curve.name.clone(), true))
        .collect::<Vec<_>>()
});
```

#### 2. Update Synchronization
```rust
// Sync derived state when props change
if current_curve_names != existing_names {
    // Update derived state while preserving user preferences
}
```

#### 3. Cleanup & Resource Management
- Automatic cleanup via Dioxus component lifecycle
- Signal cleanup handled by framework
- No manual resource management needed