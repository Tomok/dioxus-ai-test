# CLAUDE-GENERAL.md - Radar Graph Feature Module

## Radar Graph Visualization System

### Feature Overview
The radar component system represents the core functionality of this application - an interactive, responsive radar graph visualization with advanced user interaction capabilities. This module demonstrates complex state management, mathematical computation, and sophisticated UI patterns.

### System Architecture

#### 1. Component Hierarchy
```
radar/
├── container.rs              # Main feature container & orchestration
└── container/
    ├── graph/               # Visualization engine
    │   └── radar.rs        # Core SVG rendering & interaction
    │       ├── axis.rs     # Axis lines and labels
    │       ├── grid.rs     # Background grid system
    │       ├── curve.rs    # Data curve rendering
    │       └── curve/
    │           └── data_point.rs  # Interactive data points
    └── legend.rs           # Curve visibility controls
```

#### 2. Responsibility Distribution
**Container Layer (`container.rs`):**
- Overall feature state management
- Component coordination and communication
- Data validation and error handling
- Responsive layout management

**Visualization Layer (`graph/radar.rs`):**
- SVG rendering and coordinate systems
- Mathematical calculations for positioning
- User interaction event handling
- Tooltip state management

**Component Layer (axis, grid, curve, data_point):**
- Specialized rendering for specific visual elements
- Focused interaction handling
- Optimized performance for specific tasks

### Data Flow & State Management

#### 1. Multi-Dimensional State Architecture
**State Separation by Concern:**
```rust
// What to display (data state)
let mut props_signal = use_signal(|| props);

// How to display it (UI state)
let mut visible_map = use_signal(|| visibility_state);

// User interactions (interaction state)  
let mut tooltip_state = use_signal(|| None::<TooltipData>);
```

#### 2. State Synchronization Patterns
**Reactive Updates:**
- Props changes trigger data re-validation
- Visibility changes filter displayed curves
- Tooltip state updates affect all data points
- User edits propagate back to data layer

**Event Coordination:**
```rust
// Multi-source event handling
Legend Click -> Visibility Update -> Curve Filter -> Re-render
Data Point Hover -> Tooltip Show -> State Update -> UI Update
Value Edit -> Data Validation -> Props Update -> Cascade Re-render
```

### User Interaction Design

#### 1. Interaction Hierarchy
**Progressive Enhancement Model:**
1. **Static Display**: Radar graph with data curves
2. **Hover Feedback**: Tooltips reveal data point information
3. **Click Interaction**: Pin tooltips for persistent display
4. **Edit Capability**: Double-click pinned tooltips to edit values
5. **Control Interface**: Legend toggles curve visibility

#### 2. Touch & Mouse Support
**Universal Interaction Patterns:**
- Mouse hover for desktop tooltip display
- Click/tap for mobile and desktop tooltip pinning
- Double-click/double-tap for editing mode
- Keyboard navigation for accessibility

#### 3. Feedback Systems
**Visual Feedback:**
- Immediate hover responses with cursor changes
- Pinned tooltips have visual indicators (border styling)
- Legend items show disabled state (opacity, strikethrough)
- Smooth transitions for all state changes

### Mathematical & Geometric Implementation

#### 1. Coordinate System Management
**Polar to Cartesian Conversion:**
```rust
// Convert radar graph coordinates to SVG coordinates
let angle = -PI / 2.0 + i as f32 * axis_angle_step;
let point_radius = radius * (value / max_value).clamp(0.0, 1.0);
let (x, y) = polar_to_cartesian(radius, angle, center_x, center_y);
```

#### 2. Curve Generation Algorithms
**Smooth Curve Rendering:**
- Cubic Bezier curves for smooth connections between data points
- Control point calculation based on tangent angles
- Optimized path generation for SVG rendering
- Proper curve closure for filled polygons

#### 3. Scale & Positioning Calculations
**Responsive Scaling:**
```rust
let center_x = width as f32 / 2.0;
let center_y = height as f32 / 2.0;
let radius = f32::min(center_x, center_y) * 0.8;  // 80% of available space
```

### Responsive Design Implementation

#### 1. Layout Adaptation
**Screen Size Responses:**
```rust
// Mobile: Vertical stacking
class: "flex flex-col md:flex-row justify-center items-center md:items-start gap-4"

// Desktop: Horizontal layout with sidebar
```

#### 2. Component Sizing Strategy
**Fixed vs Flexible Elements:**
- **Graph**: Fixed aspect ratio for consistent visualization
- **Legend**: Flexible sizing with proper spacing
- **Tooltips**: Position adaptation based on viewport

#### 3. Touch Optimization
**Mobile-Friendly Interactions:**
- Appropriate touch target sizing for data points
- Prevent zoom on double-tap editing
- Touch-friendly tooltip positioning
- Gesture-compatible interaction patterns

### Performance Optimization Techniques

#### 1. Rendering Optimization
**Efficient SVG Generation:**
- Pre-computed coordinate arrays for multiple uses
- Minimal SVG path recalculation
- Strategic use of SVG groups for organization
- Optimized pointer-events management

#### 2. State Update Efficiency
**Minimized Re-renders:**
```rust
// Strategic signal access patterns
let props_read = props_signal.read();  // Single read
let data = props_read.some_field();    // Reuse read reference
```

#### 3. Memory Management
**Efficient Data Structures:**
- Vec<T> for dynamic data with efficient indexing
- Strategic cloning vs reference usage
- Minimal heap allocations in hot paths

### Error Handling & Data Validation

#### 1. Input Validation Strategy
**Comprehensive Data Validation:**
```rust
// Validate data consistency
if curve.data_points.len() != axes.len() {
    return Err(RadarError::DataPointCountMismatch {
        curve_name: curve.name.clone(),
        expected: axes.len(),
        actual: curve.data_points.len(),
    });
}
```

#### 2. Runtime Error Recovery
**Graceful Degradation:**
- Invalid data renders error messages instead of crashing
- Missing data shows appropriate placeholder content
- Calculation errors default to safe fallback values

#### 3. User Input Validation
**Edit Value Validation:**
- Numeric input validation for data point editing
- Range checking (0-100 scale enforcement)
- Real-time feedback for invalid inputs
- Graceful handling of edge cases

### Accessibility Implementation

#### 1. Keyboard Navigation
**Full Keyboard Support:**
- Tab navigation through interactive elements
- Enter/Space activation for buttons and controls
- Escape key cancellation for edit operations
- Arrow key navigation where appropriate

#### 2. Screen Reader Support
**Semantic Structure:**
- Proper SVG title and description elements
- ARIA labels for interactive components
- Logical tab order for complex interactions
- Alternative text for visual information

#### 3. High Contrast Support
**Visual Accessibility:**
- Dark mode with sufficient contrast ratios
- Clear focus indicators for all interactive elements
- Color-blind friendly curve color selection
- Scalable text and UI elements

### Integration Points

#### 1. External Data Integration
**Data Source Flexibility:**
- Props-based data input allows any data source
- Validation layer ensures data consistency
- Error handling for malformed data
- Future-ready for API integration

#### 2. Theme System Integration
**Consistent Theming:**
- SVG elements respect global theme settings
- Custom CSS integration for specialized interactions
- Theme transition animations for smooth UX

#### 3. Platform Integration
**Cross-Platform Considerations:**
- SVG rendering compatibility across platforms
- Touch vs mouse interaction handling
- Platform-specific performance optimizations

### Feature Extension Points

#### 1. Data Enhancement Opportunities
**Additional Data Types:**
- Support for different value ranges (beyond 0-100)
- Time-series data with animation capabilities
- Multiple scales on same axis
- Data grouping and categorization

#### 2. Interaction Enhancements
**Advanced User Features:**
- Drag-and-drop data point repositioning
- Multi-select operations
- Undo/redo functionality
- Keyboard shortcuts for power users

#### 3. Visualization Extensions
**Chart Type Variations:**
- Different curve interpolation methods
- Alternative chart types (star, web, polar area)
- Animated transitions between datasets
- 3D visualization options

### Testing & Quality Assurance

#### 1. Component Testing Strategy
**Multi-Level Testing:**
- Unit testing for mathematical functions
- Component testing for UI interactions
- Integration testing for state management
- Visual regression testing for rendering

#### 2. User Experience Testing
**Interaction Validation:**
- Cross-device interaction testing
- Accessibility compliance verification
- Performance testing with large datasets
- Browser compatibility validation

#### 3. Edge Case Handling
**Robustness Testing:**
- Empty data set handling
- Single data point edge cases
- Maximum/minimum value boundaries
- Rapid user interaction scenarios

This radar graph system showcases sophisticated software architecture applied to data visualization, demonstrating how complex interactive features can be built with clean, maintainable, and performant code using modern Rust UI development practices.