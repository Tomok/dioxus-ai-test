# CLAUDE-GENERAL.md

## Dioxus Radar Graph Application - Project Overview

### Project Summary
This is an interactive radar graph visualization application built with Dioxus, a modern Rust framework for building cross-platform user interfaces. The application demonstrates advanced data visualization techniques, responsive design, and sophisticated state management patterns.

**Key Features:**
- Interactive radar/spider graph visualization
- Real-time data editing via tooltip interactions
- Dark/light theme switching with system preference detection
- Responsive design optimized for mobile and desktop
- Curve visibility toggling via interactive legend
- Cross-platform deployment (web, desktop, mobile)

**Live Demo:** https://tomok.github.io/dioxus-ai-test/

### Architecture Overview

#### 1. Technology Stack
**Core Framework:**
- **Dioxus 0.6.0**: React-like UI framework for Rust
- **RSX Macro**: JSX-like syntax for component templates
- **Signal-based reactivity**: Efficient state management and re-rendering

**Styling & Design:**
- **Tailwind CSS 3.3.5**: Utility-first CSS framework
- **Class-based dark mode**: JavaScript-controlled theme switching
- **Custom SVG styling**: Specialized styles for interactive graphics

**Development Environment:**
- **Nix Flakes**: Reproducible development environment
- **Rust stable**: Latest stable toolchain with WASM target
- **Cross-platform builds**: Web, desktop, and mobile support

#### 2. Application Structure
```
/
├── src/
│   ├── main.rs                     # Application entry point & root component
│   ├── components.rs               # Component module registry
│   └── components/
│       ├── theme_buttons.rs        # Theme switching controls
│       ├── tooltip.rs              # Basic tooltip component
│       ├── editable_tooltip.rs     # Interactive tooltip with editing
│       ├── utils.rs                # Shared utility functions
│       └── radar/                  # Radar graph components
│           ├── container.rs        # Main radar container & state management
│           └── container/
│               ├── graph/          # SVG visualization components
│               │   └── radar/      # Core radar graph implementation
│               │       ├── axis.rs # Axis lines and labels
│               │       ├── grid.rs # Background grid circles
│               │       ├── curve.rs# Data curve visualization
│               │       └── curve/
│               │           └── data_point.rs # Interactive data points
│               └── legend.rs       # Interactive curve legend
├── assets/
│   ├── styling/
│   │   └── main.css               # Custom CSS and theme foundation
│   └── tailwind.css               # Compiled Tailwind utilities
├── flake.nix                      # Nix development environment
├── Cargo.toml                     # Rust dependencies and features
├── Dioxus.toml                    # Dioxus build configuration
└── tailwind.config.js             # Tailwind CSS configuration
```

### Core Features & Functionality

#### 1. Interactive Radar Graph
**Visualization Capabilities:**
- Multiple overlapping data curves with distinct colors
- Smooth curved connections between data points (cubic Bezier)
- Background grid with scale indicators (0-100)
- Axis labels with proper text positioning
- Responsive SVG scaling for different screen sizes

**Interaction Features:**
- **Hover tooltips**: Display data point values on mouse hover
- **Click-to-pin**: Pin tooltips for persistent display
- **Inline editing**: Double-click pinned tooltips to edit values
- **Real-time updates**: Changes immediately reflected in visualization
- **Curve toggling**: Show/hide curves via legend interaction

#### 2. Advanced State Management
**State Architecture:**
```rust
// Multi-dimensional state management
let mut props_signal = use_signal(|| props);           // Data state
let mut visible_map = use_signal(|| visibility_state); // UI state  
let mut tooltip_state = use_signal(|| None);           // Interaction state
```

**State Synchronization:**
- Reactive updates cascade through component hierarchy
- Visibility changes filter rendered curves
- Tooltip state shared across all data points
- Props mutations trigger re-renders throughout the tree

#### 3. Theme System
**Dark/Light Mode Features:**
- **System preference detection**: Automatically matches OS theme
- **Manual override**: User can force light or dark mode
- **Persistent preferences**: Settings saved to localStorage
- **Smooth transitions**: 300ms CSS transitions between themes
- **Platform compatibility**: Web-only feature with graceful fallbacks

**Implementation:**
```javascript
// Theme detection and application
if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark');
}
```

#### 4. Responsive Design
**Layout Strategy:**
- **Mobile-first**: Vertical stacking of components
- **Tablet/Desktop**: Horizontal layout with sidebar legend
- **Flexible sizing**: Graph maintains aspect ratio, legend adapts
- **Touch-friendly**: Appropriate spacing and sizing for mobile interaction

**Breakpoint Implementation:**
```rust
// Responsive flex layout
class: "flex flex-col md:flex-row justify-center items-center md:items-start gap-4"
```

### User Experience Design

#### 1. Interaction Patterns
**Progressive Enhancement:**
1. **View**: Basic static radar graph
2. **Hover**: Tooltips reveal detailed information
3. **Click**: Pin tooltips for persistent viewing
4. **Edit**: Inline editing of data values
5. **Control**: Legend interaction for curve visibility

#### 2. Accessibility Features
- **Keyboard navigation**: Focus management for interactive elements
- **High contrast**: Dark mode provides improved contrast ratios
- **Screen reader support**: Semantic HTML structure with proper ARIA attributes
- **Touch accessibility**: Appropriate sizing for mobile touch targets

#### 3. Performance Optimizations
**Rendering Efficiency:**
- Signal-based reactivity minimizes unnecessary re-renders
- SVG path pre-computation for smooth animations
- Efficient data structures for large datasets
- Strategic use of memoization for expensive calculations

**Loading Performance:**
- Single CSS bundle with minimal size
- Compile-time asset bundling via Dioxus asset system
- Optimized JavaScript for theme switching functionality

### Development Workflow

#### 1. Development Commands
```bash
# Setup development environment
nix develop

# Development server (NOT for Claude usage)
dx serve

# Build for production
dx build

# Compile Tailwind CSS
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css

# Code quality (MANDATORY before commits)
cargo fmt
cargo clippy
```

#### 2. Git Workflow & Quality Control
**Pre-commit Requirements:**
- Code formatting via `cargo fmt`
- Lint checking via `cargo clippy`
- All clippy issues must be resolved before commit

**AI-Generated Commits:**
- Commits generated with AI assistance are clearly marked
- Co-authored by Claude for transparency
- Include detailed commit messages explaining changes

#### 3. Deployment Pipeline
**GitHub Pages Integration:**
- Automatic deployment on push to main branch
- GitHub Actions build and deploy workflow
- Optimized build with proper base path configuration
- Live updates at https://tomok.github.io/dioxus-ai-test/

### Data Model & API Design

#### 1. Core Data Structures
```rust
// Data point representation
#[derive(Clone, PartialEq)]
pub struct DataPoint {
    pub value: f32,    // Numeric value (0-100 scale)
    pub label: String, // Axis label
}

// Curve/dataset representation  
#[derive(Clone, PartialEq)]
pub struct RadarCurve {
    pub name: String,              // Display name
    pub data_points: Vec<DataPoint>, // Ordered data points
    pub color: String,             // Hex color code
}

// Tooltip interaction state
#[derive(Clone, PartialEq)]
pub struct TooltipData {
    pub curve_index: usize,  // Which curve
    pub point_index: usize,  // Which point
    pub label: String,       // Display text
    pub value: f32,          // Current value
    pub x: f32, pub y: f32,  // Screen coordinates
    pub color: String,       // Curve color
    pub pinned: bool,        // Persistent display
    pub editing: bool,       // Edit mode active
}
```

#### 2. Event Handling
**Type-Safe Event System:**
```rust
// Value change events
type ValueChangeEvent = (usize, usize, f32);
//                      ^curve ^point ^new_value

// Event handler props
#[props(optional)]
pub on_value_change: Option<EventHandler<ValueChangeEvent>>,
```

### Configuration & Customization

#### 1. Build Configuration (`Dioxus.toml`)
```toml
[web.app]
title = "radar-graph"
base_path = "dioxus-ai-test"  # GitHub Pages deployment path

[web.resource]
style = []   # Additional CSS files
script = []  # Additional JavaScript files
```

#### 2. Styling Configuration (`tailwind.config.js`)
```javascript
module.exports = {
  darkMode: 'class',  // Class-based dark mode
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        'background': '#ffffff',    // Semantic colors
        'text': '#333333',         // for consistent theming
        // ... additional custom colors
      },
    },
  },
};
```

#### 3. Code Quality Configuration (`clippy.toml`)
```toml
# Dioxus-specific linting rules
await-holding-invalid-types = [
  "generational_box::GenerationalRef",
  "dioxus_signals::Write",
  # Prevents common async/await issues with Dioxus signals
]
```

### Testing & Quality Assurance

#### 1. Code Quality Tools
- **Clippy**: Advanced linting with Dioxus-specific rules
- **Rustfmt**: Consistent code formatting
- **Type checking**: Compile-time correctness verification

#### 2. Manual Testing Guidelines
**Functional Testing:**
- Verify radar graph renders correctly across screen sizes
- Test theme switching in different browsers
- Validate tooltip interactions (hover, click, edit)
- Check legend visibility toggling functionality
- Ensure data editing updates persist correctly

**Cross-Platform Testing:**
- Web browsers (Chrome, Firefox, Safari, Edge)
- Mobile devices (iOS Safari, Android Chrome)
- Desktop platforms (Linux, macOS, Windows via WSL)

### Deployment & Distribution

#### 1. Web Deployment
**GitHub Pages:**
- Automatic builds on push to main
- Optimized asset bundling
- CDN distribution for global performance
- HTTPS by default

#### 2. Alternative Deployment Options
**Desktop Applications:**
```bash
# Build desktop application
dx build --platform desktop
```

**Mobile Applications:**
```bash  
# Build mobile application (requires additional setup)
dx build --platform mobile
```

### Maintenance & Future Development

#### 1. Regular Maintenance Tasks
- Update dependencies (Dioxus, Rust toolchain, npm packages)
- Monitor performance and bundle size
- Update Tailwind CSS for new utilities and bug fixes
- Review and update documentation

#### 2. Potential Enhancements
**Feature Additions:**
- Data import/export functionality
- Additional chart types (bar, line, pie)
- Animation transitions between data sets
- Print/export functionality
- Collaborative editing features

**Technical Improvements:**
- WebAssembly performance optimizations
- Service worker for offline functionality
- Progressive Web App (PWA) features
- Advanced accessibility improvements

### Troubleshooting Guide

#### 1. Common Development Issues
**Build Failures:**
- Check Rust toolchain version compatibility
- Verify wasm-bindgen version matching between Cargo.toml and environment
- Ensure all system dependencies are installed via Nix

**Styling Issues:**
- Recompile Tailwind CSS after class changes
- Check for conflicting custom CSS and utility classes
- Verify dark mode class application

**Runtime Errors:**
- Check browser console for JavaScript errors
- Verify SVG rendering compatibility across browsers
- Test theme switching localStorage functionality

#### 2. Performance Troubleshooting
**Slow Rendering:**
- Profile component re-renders with browser dev tools
- Check signal usage patterns for efficiency
- Verify SVG path generation performance

**Large Bundle Size:**
- Analyze Tailwind CSS purging configuration
- Review unused dependencies in Cargo.toml
- Check asset optimization settings

This project demonstrates modern Rust web development practices, showcasing the power of Dioxus for building interactive, responsive applications with sophisticated state management and cross-platform deployment capabilities.