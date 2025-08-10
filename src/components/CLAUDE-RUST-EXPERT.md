# CLAUDE-RUST-EXPERT.md - Components Directory

## Component Architecture Overview

### Component Organization Philosophy
This directory contains all reusable UI components, organized by functionality and usage patterns:

- **radar/** - Complex radar graph visualization (hierarchical components)
- **theme_buttons.rs** - Platform-specific theme controls (leaf component)
- **tooltip.rs** - Basic tooltip component (leaf component) 
- **editable_tooltip.rs** - Enhanced tooltip with editing capabilities (leaf component)
- **utils.rs** - Shared utility functions (not a component)

### Component Design Patterns

#### 1. Leaf vs Container Components
**Leaf Components** (single files):
- `theme_buttons.rs` - Self-contained theme switching
- `tooltip.rs` - Basic display-only tooltip
- `editable_tooltip.rs` - Interactive tooltip with state

**Container Components** (directories):
- `radar/` - Complex visualization with multiple subcomponents

#### 2. Props Design Patterns

**Flexible Props with Defaults:**
```rust
#[derive(Props, PartialEq, Clone)]
pub struct ComponentProps {
    pub required_field: String,
    #[props(default = 100.0)]
    pub optional_with_default: f32,
    #[props(optional)]  
    pub optional_callback: Option<EventHandler<T>>,
}
```

**Validation-First Props:**
- Constructor methods with `Result<Props, Error>` returns
- Validation logic encapsulated in props implementation
- Builder-like patterns for complex component configuration

#### 3. State Management Strategies

**Local State Pattern** (theme_buttons.rs):
- No persistent state needed
- Direct DOM manipulation via JavaScript
- Platform-specific implementations

**Shared State Pattern** (tooltip components):
- State passed via props from parent
- Signal-based reactivity for updates
- Parent-child communication via callbacks

**Complex State Pattern** (radar components):
- Multiple signal types for different aspects
- Computed properties derived from base state  
- State synchronization across component tree

### Component Communication Patterns

#### 1. Parent-Child Communication
**Downward (Props):**
```rust
// Data flows down
ParentComponent {
    child_prop: parent_state.read().clone(),
    callback: parent_handler
}
```

**Upward (Callbacks):**
```rust
// Events flow up
#[props(optional)]
pub on_event: Option<EventHandler<EventData>>,

// Usage in component
if let Some(handler) = &on_event {
    handler.call(event_data);
}
```

#### 2. Sibling Communication
- Via shared parent state (signals)
- Parent coordinates between children
- Event aggregation at parent level

#### 3. Global State Access
- Platform-specific features via feature flags
- Direct browser API access where needed
- Minimal global state dependencies

### Error Handling in Components

#### 1. Input Validation Strategy
```rust
// Validate props on construction
impl ComponentProps {
    pub fn new(...) -> Result<Self, ComponentError> {
        // Validation logic
        // Return error with detailed message
    }
}
```

#### 2. Runtime Error Handling
```rust
// Graceful degradation in render
if invalid_state {
    return rsx! { div { "Error: {error_message}" } };
}
```

#### 3. Error Propagation
- Errors bubble up via Result types
- UI errors handled with fallback rendering
- Business logic errors propagated to parent

### Platform-Specific Component Design

#### 1. Conditional Component Implementation
```rust
#[cfg(feature = "web")]
#[component]
pub fn PlatformComponent() -> Element {
    // Web implementation
}

#[cfg(not(feature = "web"))]
#[component] 
pub fn PlatformComponent() -> Element {
    // Fallback implementation
}
```

#### 2. Feature Detection Patterns
- Compile-time feature flags for major differences
- Runtime detection for subtle variations
- Graceful degradation for unsupported features

### Component Reusability Design

#### 1. Generic Component Patterns
- Props-based customization
- Callback-based behavior modification
- Style injection via class properties

#### 2. Composition Over Inheritance
- Small, focused components
- Composition via component nesting
- Behavior mixing via props

#### 3. API Stability
- Careful props design for backward compatibility
- Optional props for new features
- Deprecation strategy for breaking changes

### Performance Optimization Techniques

#### 1. Render Optimization
- `PartialEq` implementations on props
- Strategic use of signals vs direct props
- Minimal component re-rendering

#### 2. Memory Efficiency
- Clone vs reference trade-offs
- Strategic use of `Rc`/`Arc` where beneficial
- Efficient data structure choices

#### 3. Computation Optimization
- Cached computed properties
- Lazy evaluation where possible
- Efficient algorithms in utility functions