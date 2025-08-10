# CLAUDE-GENERAL.md - Source Code Organization

## Source Code Structure & Patterns

### Application Architecture Overview
The source directory demonstrates a well-organized Dioxus application with clear separation of concerns, modular component design, and efficient resource management.

### File Organization Strategy

#### 1. Entry Point Design (`main.rs`)
**Application Bootstrap:**
- Asset management with compile-time bundling
- Sample data creation and initialization  
- Platform-specific feature handling
- Root component with layout and theming

**Key Responsibilities:**
```rust
// Asset declarations
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Sample data creation
let curve1 = RadarCurve { /* Model A data */ };
let curve2 = RadarCurve { /* Model B data */ };

// Platform-specific initialization
#[cfg(feature = "web")]
use_effect(|| { /* Theme initialization */ });
```

#### 2. Component Registry (`components.rs`)
**Module Management:**
- Central import point for all UI components
- Documentation at module level
- Selective re-exports for clean API surface

**Design Philosophy:**
- Components exported directly from their modules
- No complex re-export hierarchies
- Clear module boundaries and responsibilities

#### 3. Component Organization Patterns
**Hierarchical Structure:**
```
components/
├── theme_buttons.rs        # Platform-specific UI controls
├── tooltip.rs             # Basic display components
├── editable_tooltip.rs    # Enhanced interactive components  
├── utils.rs              # Shared utility functions
└── radar/                # Complex feature modules
    ├── container.rs      # Main feature container
    └── container/        # Sub-feature components
        ├── graph/        # Visualization components
        └── legend.rs     # Supporting UI components
```

**Component Categories:**
- **Leaf Components**: Single-file, focused functionality
- **Container Components**: Directory-based, multiple related components  
- **Utility Modules**: Shared functions, not components

### Development Patterns

#### 1. Feature Flag Usage
**Platform Abstraction:**
```rust
#[cfg(feature = "web")]
// Web-specific implementation with browser APIs

#[cfg(not(feature = "web"))]  
// Fallback implementation for other platforms
```

**Benefits:**
- Compile-time optimization for target platforms
- Clean separation of platform-specific code
- API compatibility across all deployment targets

#### 2. Data Flow Architecture
**Sample Data Strategy:**
- Hard-coded demo data in main.rs for immediate functionality
- Structured data types demonstrate real-world usage patterns
- Easy to replace with dynamic data loading in future versions

**State Management:**
- Root component owns sample data
- State passed down through props
- Child components communicate up via callbacks

#### 3. Asset Management
**Compile-Time Asset Bundling:**
```rust
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Usage in component
document::Link { rel: "stylesheet", href: TAILWIND_CSS }
```

**Benefits:**
- Path resolution handled at compile time
- Automatic asset optimization and bundling
- Cross-platform compatibility for asset references

### Code Quality & Maintenance

#### 1. Documentation Standards
**Module-Level Documentation:**
```rust
//! The components module contains all shared components for our app.
//! Components are the building blocks of dioxus apps.
```

**Component Documentation:**
- Clear purpose statements
- Props documentation
- Usage examples where appropriate
- Platform compatibility notes

#### 2. Error Handling Patterns
**Graceful Degradation:**
```rust
// Early returns for invalid states
if axes_count == 0 {
    return rsx! { div { "No axes provided for radar graph" } };
}
```

**Validation Strategy:**
- Input validation at component boundaries
- Clear error messages for development debugging
- Graceful UI fallbacks for runtime errors

#### 3. Type Safety Practices
**Strong Typing:**
```rust
// Explicit type definitions
#[derive(Props, PartialEq, Clone)]
pub struct ComponentProps {
    pub required_field: String,
    #[props(default = 100.0)]
    pub optional_field: f32,
}
```

**Benefits:**
- Compile-time correctness verification
- Clear API contracts between components
- Refactoring safety through type system

### Performance Considerations

#### 1. Component Re-rendering
**Optimization Strategies:**
- `PartialEq` implementations on all props
- Strategic signal usage vs direct prop passing
- Minimal component hierarchies where possible

#### 2. Memory Management
**Efficient Patterns:**
- Clone-heavy patterns acceptable due to Rust optimizations
- Strategic use of references where lifetime management is clear
- Minimal heap allocations in render paths

#### 3. Asset Loading
**Optimization Features:**
- Compile-time asset bundling reduces runtime overhead
- Single CSS file reduces HTTP requests
- Asset macro enables aggressive optimization

### Integration Points

#### 1. Platform Integration
**Web Platform:**
- Direct JavaScript integration via js_sys::eval
- DOM manipulation for theme switching
- localStorage integration for preferences

**Non-Web Platforms:**
- Graceful feature degradation
- Alternative implementation strategies
- Consistent API surface across platforms

#### 2. External Dependencies
**Minimal Dependency Strategy:**
- Core Dioxus ecosystem only
- Platform-specific dependencies behind feature flags
- Clear separation between build-time and runtime dependencies

### Development Workflow

#### 1. File Modification Patterns
**Component Development:**
1. Define props structure with validation
2. Implement component logic with error handling
3. Create RSX template with proper styling
4. Add documentation and examples

**Module Extension:**
1. Add new module to components.rs
2. Implement component with consistent patterns
3. Integrate with existing component hierarchy
4. Update documentation and usage examples

#### 2. Testing Strategy
**Component Testing:**
- Type-level testing via compilation
- Visual testing via development server
- Cross-platform compatibility testing

**Integration Testing:**  
- Full application flow testing
- Theme switching functionality
- Responsive design validation

#### 3. Refactoring Guidelines
**Safe Refactoring:**
- Leverage Rust's type system for correctness
- Use compiler errors to guide changes
- Maintain API compatibility during iterations

**Component Evolution:**
- Start with simple implementations
- Add complexity incrementally
- Maintain clear component boundaries

### Best Practices Summary

#### 1. Code Organization
- Keep components focused on single responsibilities
- Use clear, descriptive naming conventions
- Maintain consistent file organization patterns
- Document component APIs and usage patterns

#### 2. Performance
- Implement PartialEq on all props structures
- Use signals judiciously for state management
- Minimize unnecessary re-renders through design
- Leverage compile-time optimizations

#### 3. Maintainability  
- Write self-documenting code with clear intent
- Use type safety to prevent runtime errors
- Maintain consistent error handling patterns
- Keep platform-specific code clearly isolated

#### 4. Collaboration
- Follow consistent coding style and patterns
- Document complex logic and architectural decisions
- Use descriptive commit messages with AI attribution
- Maintain clear separation between different concerns