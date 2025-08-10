# CLAUDE-RUST-EXPERT.md - Source Code Structure

## Source Code Architecture

### Module Organization Strategy
This directory follows a hierarchical module structure that reflects the component relationships and usage patterns:

- `main.rs` - Application entry point and root component
- `components.rs` - Central module registry for all UI components
- `components/` - Individual component implementations

### Key Rust Patterns in Source Structure

#### 1. Main Application Pattern (`main.rs`)
**Entry Point Design:**
- Uses `dioxus::launch(App)` for framework initialization
- Asset management with `asset!` macro for compile-time resource bundling
- Platform-specific initialization code with conditional compilation

**Root Component Responsibilities:**
- Sample data creation and management
- Platform-specific theme initialization
- Top-level layout and component orchestration

#### 2. Module Registry Pattern (`components.rs`)
**Centralized Export Strategy:**
- Public module declarations without re-exports
- Allows for controlled API surface
- Documentation at module level for discoverability

#### 3. Component Module Structure
**File vs Directory Decision:**
- Uses `<name>.rs` files for leaf components
- Uses `<name>/` directories for components with subcomponents
- Clear hierarchy: `radar/container/graph/radar/curve/data_point.rs`

### Data Flow Architecture

#### 1. Prop Threading Pattern
```rust
// Data flows down from main.rs through component hierarchy
App -> RadarContainer -> RadarGraph -> RadarCurveVisual -> DataPoint
```

#### 2. Signal-Based State Management
- Root component creates immutable sample data
- Container manages mutable state via signals
- State changes propagate through signal subscriptions

#### 3. Event Bubbling Pattern
- Events bubble up through callback props
- Type-safe event data via tuples and custom types
- Optional event handlers for flexible component APIs

### Error Handling Strategy

#### 1. Validation at Boundaries
- Input validation in main.rs (sample data creation)
- Component-level validation with graceful degradation
- Error types defined close to usage (in container module)

#### 2. Early Return Pattern
```rust
if axes_count == 0 {
    return rsx! { div { "No axes provided" } };
}
```

#### 3. Result<T, E> for Data Operations
- Constructor validation returns Results
- Business logic failures handled explicitly
- UI components handle errors with fallback rendering

### Platform Abstraction Implementation

#### 1. Conditional Compilation Strategy
```rust
#[cfg(feature = "web")]
use_effect(|| { /* web-specific initialization */ });
```

#### 2. Feature Flag Organization
- Web-specific code isolated to specific functions/components
- Non-web builds exclude unnecessary code paths
- Runtime feature detection where needed

### Component Lifecycle Patterns

#### 1. Initialization Pattern
- `use_effect` for side effects
- `use_signal` for reactive state
- Props processing in component body

#### 2. Update Patterns
- Signal mutations trigger re-renders
- Props changes cause component re-evaluation
- Conditional rendering based on state

#### 3. Cleanup Patterns
- Automatic cleanup via Dioxus lifecycle
- No manual memory management needed
- Signal cleanup handled by framework