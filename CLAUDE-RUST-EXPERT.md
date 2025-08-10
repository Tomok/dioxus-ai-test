# CLAUDE-RUST-EXPERT.md

## Dioxus Radar Graph Application - Rust Architecture Documentation

### Overview
This Dioxus-based application demonstrates advanced Rust patterns for building reactive web applications. The architecture follows component-based design principles similar to React, but leverages Rust's type system and ownership model for memory safety and performance.

### Core Rust Patterns & Architecture

#### 1. Component System Architecture
The application uses Dioxus's component system with several key patterns:

**Component Definition Pattern:**
```rust
#[component]
fn ComponentName(props: ComponentProps) -> Element {
    // Component logic
    rsx! { /* JSX-like syntax */ }
}
```

**Props System:**
- Uses `#[derive(Props)]` for component properties
- Implements `PartialEq` for efficient re-rendering
- Uses `#[props(default)]` and `#[props(optional)]` for flexible APIs

#### 2. State Management Patterns

**Signal-Based Reactivity:**
- Uses `use_signal()` for reactive state management
- Signals automatically trigger re-renders when mutated
- Shared state between components via signal passing

**Key State Patterns:**
```rust
// Mutable state with automatic reactivity
let mut props_signal = use_signal(|| props);

// Derived state from multiple signals
let visible_curves = /* computed from props_signal and visible_map */;

// Shared state between components
let mut tooltip_state = use_signal(|| None::<TooltipData>);
```

#### 3. Type System & Data Modeling

**Core Data Structures:**
- `DataPoint`: Represents a single value-label pair
- `RadarCurve`: Contains name, color, and vector of data points
- `TooltipData`: Complex state object with positioning and editing state
- `RadarError`: Comprehensive error types using `thiserror`

**Type Safety Patterns:**
- Extensive use of `Result<T, E>` for error handling
- Custom error types with detailed error messages
- Validation at component boundaries

#### 4. Error Handling Strategy

**Structured Error Types:**
```rust
#[derive(Error, Debug, Clone)]
pub enum RadarError {
    #[error("Data point count mismatch in curve '{curve_name}': expected {expected} points, got {actual} points")]
    DataPointCountMismatch { curve_name: String, expected: usize, actual: usize },
    // ... more variants
}
```

**Validation Patterns:**
- Input validation in constructors and setters
- Early returns for invalid states
- Graceful degradation in UI components

#### 5. Platform Abstraction

**Feature Flag Pattern:**
```rust
#[cfg(feature = "web")]
// Web-specific implementation

#[cfg(not(feature = "web"))]
// Non-web fallback
```

Used extensively for platform-specific functionality like theme switching.

### Component Hierarchy & Communication

#### 1. Component Tree Structure
```
App (root)
├── ThemeButtons (platform-conditional)
└── RadarContainer (main container)
    ├── RadarGraph (SVG visualization)
    │   ├── RadarGrid (background grid)
    │   ├── RadarAxis (axis lines and labels)
    │   ├── RadarCurveVisual[] (data curves)
    │   │   └── DataPoint[] (interactive points)
    │   └── EditableTooltip (conditional)
    └── RadarLegend (curve visibility controls)
```

#### 2. Data Flow Patterns

**Top-Down Data Flow:**
- Props cascade from parent to child components
- Immutable data structures passed as props
- Signal-based reactivity for state updates

**Bottom-Up Communication:**
- Event handlers passed as props (`EventHandler<T>`)
- Callback pattern for child-to-parent communication
- Signal mutation for shared state updates

**Event Handler Pattern:**
```rust
#[props(optional)]
pub on_value_change: Option<EventHandler<(usize, usize, f32)>>,

// Usage in child component
if let Some(callback) = &on_value_change {
    callback.call((curve_index, point_index, new_value));
}
```

### Advanced Rust Techniques

#### 1. Lifetime Management
- No explicit lifetime annotations needed due to Dioxus's ownership model
- Components own their props, eliminating borrowing complexity
- Signal-based state eliminates most lifetime issues

#### 2. Memory Management
- Zero-copy operations where possible
- Clone-heavy patterns acceptable due to Rust's optimization
- Careful use of `Vec<T>` and `String` for dynamic data

#### 3. Functional Programming Patterns
- Extensive use of iterators and closures
- Map/filter/collect chains for data transformation
- Immutable data structures with controlled mutation points

#### 4. Type-Driven Development
- Rich type definitions guide implementation
- Compiler-enforced correctness
- Pattern matching for state transitions

### Performance Considerations

#### 1. Re-render Optimization
- `PartialEq` implementations for props to prevent unnecessary renders
- Signal-based updates only re-render dependent components
- Computed properties derived from signals

#### 2. Memory Efficiency
- Strategic use of `Clone` vs references
- Signal sharing to avoid data duplication
- Efficient vector operations for data processing

#### 3. Platform-Specific Optimizations
- Web-specific features conditionally compiled
- Different code paths for different targets
- Minimal runtime overhead for unused features

### Key Design Decisions

#### 1. Signal-Heavy Architecture
- Chosen over more complex state management for simplicity
- Enables fine-grained reactivity
- Easy to reason about data flow

#### 2. Error-First Design
- Comprehensive error types defined upfront
- Validation at component boundaries
- Graceful error handling in UI

#### 3. Platform Abstraction Strategy
- Feature flags for platform differences
- Compile-time optimization for target platforms
- API compatibility across platforms

#### 4. Component Granularity
- Fine-grained components for reusability
- Clear separation of concerns
- Minimal coupling between components

### Integration Points

#### 1. Web APIs
- Direct JavaScript integration via `js_sys::eval`
- Web-specific features behind feature flags
- Local storage integration for persistence

#### 2. External Dependencies
- Minimal external crates (dioxus, web-sys, js-sys, thiserror)
- Platform-specific dependencies handled by build system
- No runtime dependencies for core functionality

### Testing & Maintainability

#### 1. Type Safety as Testing
- Extensive compile-time checking
- Error types ensure edge cases are handled
- Props validation prevents runtime errors

#### 2. Code Organization
- Clear module hierarchy
- Logical separation of concerns
- Minimal circular dependencies

#### 3. Documentation Patterns
- Comprehensive doc comments on public APIs
- Error message design for debuggability
- Type annotations for clarity