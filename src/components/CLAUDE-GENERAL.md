# CLAUDE-GENERAL.md - Component System Overview

## Component Architecture & Design Patterns

### Component Ecosystem Overview
This directory contains a comprehensive set of UI components that demonstrate modern Dioxus development patterns, from simple utility components to complex interactive visualizations.

### Component Classification

#### 1. UI Control Components
**Theme Buttons (`theme_buttons.rs`)**
- **Purpose**: Light/dark/system theme switching controls
- **Complexity**: Medium (platform-conditional implementation)
- **Features**: Platform abstraction, JavaScript integration, persistent preferences
- **Usage**: Header controls for application-wide theme management

#### 2. Display Components  
**Basic Tooltip (`tooltip.rs`)**
- **Purpose**: Simple hover-based information display
- **Complexity**: Low (straightforward SVG rendering)
- **Features**: Dark mode support, positioning, visibility controls
- **Usage**: Basic information display for data points

#### 3. Interactive Components
**Editable Tooltip (`editable_tooltip.rs`)**
- **Purpose**: Advanced tooltip with inline editing capabilities
- **Complexity**: High (complex state management, form handling)
- **Features**: Click-to-pin, inline editing, value validation, keyboard handling
- **Usage**: Primary interaction mechanism for data point modification

#### 4. Visualization System
**Radar Component Hierarchy (`radar/`)**
- **Purpose**: Complete radar graph visualization system
- **Complexity**: Very High (multi-component system)
- **Features**: Interactive data visualization, responsive design, state synchronization
- **Usage**: Core application feature

#### 5. Utility Modules
**Utils (`utils.rs`)**
- **Purpose**: Shared mathematical and geometric functions
- **Complexity**: Low (pure functions)
- **Features**: Coordinate transformations, calculations
- **Usage**: Supporting functions for visualization components

### Component Design Philosophy

#### 1. Single Responsibility Principle
**Clear Purpose Definition:**
- Each component has a well-defined, single responsibility
- Components don't mix UI rendering with business logic
- Data transformation handled separately from display logic

**Example - Tooltip Components:**
- `tooltip.rs`: Display-only, minimal state
- `editable_tooltip.rs`: Interactive editing, complex state management

#### 2. Composition Over Inheritance
**Component Composition Patterns:**
```rust
// Container composes multiple specialized components
RadarContainer {
    // Graph visualization
    RadarGraph { /* visualization props */ }
    
    // Interactive legend
    RadarLegend { /* control props */ }
}
```

#### 3. Props-Based Customization
**Flexible Component APIs:**
```rust
#[derive(Props, PartialEq, Clone)]
pub struct FlexibleProps {
    // Required core functionality
    pub data: Vec<DataPoint>,
    
    // Optional customization
    #[props(default = 100.0)]
    pub max_value: f32,
    
    // Optional behavior
    #[props(optional)]
    pub on_change: Option<EventHandler<Event>>,
}
```

### State Management Patterns

#### 1. Local State Components
**Stateless Display Components:**
```rust
// Pure display components with no internal state
#[component]
fn DisplayComponent(props: Props) -> Element {
    // No use_signal calls
    // All state comes from props
    rsx! { /* render based on props */ }
}
```

#### 2. Controlled Components
**Parent-Controlled State:**
```rust
// State managed by parent, behavior controlled via props
#[component] 
fn ControlledComponent(
    data: ReadOnlySignal<Data>,
    on_change: EventHandler<Change>
) -> Element {
    // Component behavior controlled by parent
    // Changes reported back via callbacks
}
```

#### 3. Container Components
**Complex State Management:**
```rust
// Container manages multiple signals and coordinates children
#[component]
fn ContainerComponent() -> Element {
    let mut data_signal = use_signal(|| initial_data);
    let mut ui_state = use_signal(|| ui_initial);
    let mut interaction_state = use_signal(|| interaction_initial);
    
    // Complex coordination logic
    // State distribution to children
    // Event handling and state synchronization
}
```

### Component Communication Patterns

#### 1. Downward Data Flow
**Props Cascading:**
```rust
// Data flows down through component hierarchy
App -> RadarContainer -> RadarGraph -> RadarCurveVisual -> DataPoint
```

**Pattern Benefits:**
- Predictable data flow
- Easy to trace data changes
- Clear component dependencies

#### 2. Upward Event Flow  
**Callback-Based Communication:**
```rust
// Events bubble up through callback props
DataPoint -> RadarCurveVisual -> RadarGraph -> RadarContainer -> App

// Type-safe event data
type DataChangeEvent = (usize, usize, f32);
on_data_change: EventHandler<DataChangeEvent>
```

#### 3. Shared State Communication
**Signal-Based Sharing:**
```rust
// Shared signals for coordinated state
let mut shared_tooltip = use_signal(|| None::<TooltipData>);

// Multiple components can read/write shared state
Component1 { tooltip_state: shared_tooltip }
Component2 { tooltip_state: shared_tooltip }
```

### User Experience Design Patterns

#### 1. Progressive Enhancement
**Interaction Hierarchy:**
1. **Static Display**: Basic information presentation
2. **Hover Feedback**: Additional information on hover
3. **Click Interaction**: Pin tooltips, toggle visibility
4. **Edit Capability**: Inline editing of values
5. **Keyboard Support**: Full accessibility

#### 2. Responsive Component Design
**Adaptive Layouts:**
```rust
// Components adapt to container size
class: "flex flex-col md:flex-row"  // Responsive layout
class: "flex-shrink-0"              // Fixed size elements
class: "w-full min-w-0"            // Flexible elements
```

#### 3. Accessibility Integration
**Built-in Accessibility:**
- Semantic HTML structure via RSX
- Proper focus management for interactive elements
- Screen reader compatible tooltips
- Keyboard navigation support

### Error Handling & Resilience

#### 1. Graceful Degradation
**Component-Level Error Handling:**
```rust
// Handle invalid props gracefully
if invalid_condition {
    return rsx! { 
        div { 
            class: "error-message",
            "Error: {error_description}" 
        } 
    };
}
```

#### 2. Data Validation
**Input Validation Patterns:**
- Props validation in constructors
- Runtime validation with user feedback
- Default values for optional parameters
- Clear error messages for developers

#### 3. Platform Compatibility
**Feature Detection:**
```rust
#[cfg(feature = "web")]
// Full-featured web implementation

#[cfg(not(feature = "web"))]  
// Fallback implementation for other platforms
```

### Performance Optimization Strategies

#### 1. Render Optimization
**Efficient Re-rendering:**
- `PartialEq` implementations prevent unnecessary renders
- Signal-based updates only affect dependent components
- Conditional rendering reduces DOM updates

#### 2. State Update Efficiency
**Optimized State Patterns:**
```rust
// Batch related state updates
let mut batch_update = Vec::new();
// ... collect changes
// Apply all at once to minimize re-renders
```

#### 3. Resource Management
**Efficient Resource Usage:**
- Minimal heap allocations in render paths
- Strategic use of clone vs references
- Efficient SVG path generation and caching

### Component Testing Strategies

#### 1. Type-Level Testing
**Compile-Time Correctness:**
- Props validation via type system
- Event handler type safety
- Component API contract enforcement

#### 2. Visual Testing
**Development-Time Validation:**
- Component rendering in development server
- Cross-browser compatibility testing
- Responsive design validation

#### 3. Integration Testing
**End-to-End Functionality:**
- Complete user workflow testing
- State synchronization validation
- Platform compatibility testing

### Maintenance & Evolution Guidelines

#### 1. Component Lifecycle Management
**Evolution Strategies:**
- Start with simple implementations
- Add complexity incrementally based on requirements
- Maintain backward compatibility through careful API design
- Deprecate features gradually with clear migration paths

#### 2. Refactoring Best Practices
**Safe Component Changes:**
- Use Rust's type system to guide refactoring
- Maintain clear component boundaries during changes
- Test component integration after modifications
- Document breaking changes and migration strategies

#### 3. Documentation Standards
**Component Documentation:**
- Clear purpose and responsibility statements
- Props documentation with examples
- Usage patterns and best practices
- Platform compatibility notes
- Performance characteristics

### Integration Patterns

#### 1. Asset Integration
**Resource Management:**
```rust
// Components can reference shared assets
const COMPONENT_STYLE: Asset = asset!("/assets/component.css");
document::Link { rel: "stylesheet", href: COMPONENT_STYLE }
```

#### 2. External Service Integration  
**API Integration Patterns:**
- Service interfaces separated from UI components
- Data loading handled at container level
- Error states managed and displayed appropriately
- Loading states for better user experience

#### 3. Platform Integration
**Cross-Platform Considerations:**
- Feature flags for platform-specific functionality
- Graceful degradation for unsupported features
- Consistent API across all platforms
- Platform-optimized implementations where beneficial

This component system demonstrates mature software engineering practices applied to modern Rust UI development, showcasing scalable architectures, maintainable code patterns, and excellent user experience design.