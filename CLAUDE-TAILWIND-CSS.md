# CLAUDE-TAILWIND-CSS.md

## Dioxus Radar Graph Application - Tailwind CSS Design System

### Overview
This application uses Tailwind CSS as the primary styling framework, with a sophisticated dark mode implementation and custom CSS for SVG-specific interactions. The design system demonstrates advanced responsive design patterns and theme switching capabilities.

### Tailwind Configuration & Setup

#### 1. Configuration Strategy (`tailwind.config.js`)
```javascript
module.exports = {
  mode: "all",                                    // Include all utilities
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],  // Dioxus-specific patterns
  darkMode: 'class',                             // Class-based dark mode switching
  theme: {
    extend: {
      colors: {
        // Custom semantic color definitions
        'background': '#ffffff',    // Light mode background
        'text': '#333333',         // Light mode text
        'grid': '#dddddd',         // Grid/border colors
        'axis': '#888888',         // Axis line colors
        'tooltip-bg': '#333333',   // Tooltip backgrounds
        'tooltip-text': '#ffffff', // Tooltip text
      },
    },
  },
  plugins: [],
};
```

**Key Design Decisions:**
- `mode: "all"` ensures comprehensive utility inclusion for Rust-generated classes
- Custom semantic colors provide consistent theming foundation
- Class-based dark mode enables JavaScript-controlled theme switching

#### 2. Build Integration
- **Source:** `tailwind.css` (utility directives)
- **Output:** `assets/tailwind.css` (compiled utilities)
- **Build Command:** `tailwindcss -i ./tailwind.css -o ./assets/tailwind.css`
- **Development:** Manual compilation (no watch mode in Claude environment)

### Dark Mode Architecture

#### 1. Theme Switching Strategy
**Class-Based Implementation:**
```rust
// JavaScript-controlled theme application
if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark');
} else {
    document.documentElement.classList.remove('dark');
}
```

**Benefits:**
- Instant theme switching without page reload
- Persistent user preference via localStorage
- Automatic system preference detection
- Fallback to system theme when no preference set

#### 2. Dark Mode Utility Pattern
```rust
// Comprehensive dark mode class usage
class: "bg-background dark:bg-gray-900 text-text dark:text-white"
class: "bg-white dark:bg-gray-800"
class: "hover:bg-gray-200 dark:hover:bg-gray-700"
```

**Patterns Used:**
- Base color + dark variant pairs
- Hover state dark mode variants
- SVG fill/stroke dark mode adaptations
- Semantic color naming with dark overrides

### Responsive Design System

#### 1. Mobile-First Breakpoint Strategy
```rust
// Progressive enhancement from mobile to desktop
class: "flex flex-col md:flex-row justify-center items-center md:items-start gap-4"
```

**Breakpoint Usage:**
- `md:` (768px+) - Primary breakpoint for layout changes
- Mobile: Vertical stacking of components
- Desktop: Horizontal layout with side-by-side arrangement

#### 2. Container & Layout Patterns
```rust
// Responsive container patterns
class: "container mx-auto px-4 py-8"           // Main app container
class: "flex-shrink-0"                         // Fixed-size components
class: "p-4 flex-shrink-0"                     // Flexible spacing with fixed shrink
```

### Component Styling Patterns

#### 1. Interactive Elements
**Button Styling Pattern:**
```rust
class: "p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
```

**Features:**
- Consistent padding (`p-2`)
- Rounded corners (`rounded-md`)
- Hover state with dark mode variant
- Smooth transitions (`transition-colors`)

#### 2. Card/Panel Components
**Card Pattern:**
```rust
class: "bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md"
```

**Design System:**
- White background with dark mode override
- Consistent padding (`p-4`)
- Larger border radius (`rounded-lg`)
- Subtle shadow for depth (`shadow-md`)

#### 3. Typography Hierarchy
```rust
class: "text-3xl font-bold"           // Main headings
class: "text-lg font-semibold mb-2"   // Section headings  
class: "text-xs"                      // Small text (tooltips)
```

**Features:**
- Semantic size progression
- Font weight differentiation
- Consistent margin patterns

### Custom CSS Integration

#### 1. SVG-Specific Styling (`assets/tailwind.css`)
```css
/* Tooltip interaction styles */
.tooltip-pinned rect {
    stroke: #ffffff;
    stroke-width: 2px;
}

.dark .tooltip-pinned rect {
    stroke: #dddddd;
}

/* SVG pointer-events management */
.tooltip { pointer-events: none; }
.radar-curve { pointer-events: visible; }
.data-point-hitarea { pointer-events: all; }
```

**Purpose:**
- Handle SVG-specific interactions not covered by Tailwind
- Manage pointer-events for complex SVG hierarchies  
- Provide pinned state visual indicators

#### 2. Form Input Customizations
```css
/* Remove spinner arrows from number inputs */
.editable-tooltip input[type=number]::-webkit-outer-spin-button,
.editable-tooltip input[type=number]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}
```

### Advanced Styling Techniques

#### 1. Dynamic Class Composition
```rust
// Runtime class composition
let pinned_class = if props.pinned { "tooltip-pinned" } else { "" };
class: "tooltip {pinned_class}"
```

#### 2. Conditional Rendering Styles
```rust
// State-based styling
let (rect_classes, text_classes) = if is_visible {
    ("", "")
} else {
    ("opacity-30", "line-through opacity-70")
};
```

#### 3. Semantic Color Usage
```rust
// Custom color integration
class: "bg-background dark:bg-gray-900 text-text dark:text-white"
```

### Performance Optimizations

#### 1. Utility Optimization
- Only necessary utilities included via content scanning
- Custom colors reduce utility bloat
- Efficient dark mode implementation without duplication

#### 2. CSS-in-RSX Pattern
```rust
// Inline styles for dynamic values mixed with utilities
rect {
    x: "{tooltip_x - 60.0}",
    y: "{tooltip_y - 25.0}",
    class: "dark:fill-gray-800",  // Static utilities
}
```

### Accessibility Features

#### 1. Focus Management
```css
.focus\:border-blue-400:focus {
    border-color: rgb(96 165 250);
}
```

#### 2. High Contrast Support
- Dark mode provides sufficient contrast ratios
- Custom colors designed for accessibility
- Hover states clearly distinguish interactive elements

### Design System Guidelines

#### 1. Color Palette Strategy
**Light Mode:**
- Background: `#ffffff` (white)
- Text: `#333333` (dark gray)
- Accents: Tailwind gray scale

**Dark Mode:**
- Background: `#0f1116` (custom dark)
- Text: `#ffffff` (white)  
- Surfaces: Tailwind gray-800/900

#### 2. Spacing System
- Uses Tailwind's 4px-based spacing scale
- Consistent padding: `p-2` (8px), `p-4` (16px)
- Consistent gaps: `gap-2` (8px), `gap-4` (16px)
- Consistent margins: `mb-2` (8px), `mb-6` (24px)

#### 3. Typography Scale
- Heading: `text-3xl font-bold` (30px, bold)
- Subheading: `text-lg font-semibold` (18px, 600)
- Body: Default browser size
- Small: `text-xs` (12px)

### Integration with Dioxus

#### 1. RSX Class Attribute Pattern
```rust
// Clean class attribute usage in RSX
div {
    class: "container mx-auto px-4 py-8",
    // component content
}
```

#### 2. Dynamic Class Generation
```rust
// String interpolation for dynamic classes
class: "editable-tooltip {pinned_class}"
```

#### 3. Asset Integration
```rust
// Compile-time asset inclusion
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
document::Link { rel: "stylesheet", href: TAILWIND_CSS }
```

### Maintenance & Development

#### 1. Development Workflow
1. Modify Tailwind classes in Rust components
2. Run `tailwindcss -i ./tailwind.css -o ./assets/tailwind.css`
3. Test in browser with hot reloading via `dx serve`

#### 2. Customization Patterns
- Extend theme in `tailwind.config.js` for new semantic colors
- Add custom CSS in `assets/tailwind.css` for complex interactions
- Use utility classes for 90% of styling needs

#### 3. Performance Monitoring
- Monitor generated CSS size via build output
- Use content purging to eliminate unused utilities
- Optimize custom CSS for minimal specificity conflicts