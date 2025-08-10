# CLAUDE-TAILWIND-CSS.md - Component Styling Patterns

## Component-Level Styling Architecture

### Interactive Component Patterns

#### 1. Theme Button Component Styling
```rust
// Container layout
class: "flex gap-2 items-center"

// Individual button pattern
class: "p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
```

**Design System Features:**
- **Layout**: Horizontal flex with 8px gaps
- **Button Base**: 8px padding, 6px rounded corners
- **Interaction**: Hover states with dark mode variants
- **Animation**: Color transition for smooth theme switching

#### 2. Tooltip Component Styling

**Basic Tooltip Pattern:**
```rust
class: "tooltip {pinned_class}"
```

**SVG Rectangle Styling:**
```rust
class: "dark:fill-gray-800"
```

**Text Content Styling:**
```rust
class: "dark:fill-white"
```

**Features:**
- Dynamic class composition for state management
- SVG-specific dark mode adaptations
- Custom CSS integration for pinned state indicators

#### 3. Editable Tooltip Advanced Patterns

**Container Styling:**
```rust
class: "editable-tooltip {pinned_class}"
```

**Layout Management:**
```rust
class: "w-full h-full flex items-center justify-center px-2 text-white text-xs"
```

**Input Field Pattern:**
```rust
class: "flex-1 min-w-0 px-1 text-center text-white border border-gray-500 rounded outline-none focus:border-blue-400 bg-white bg-opacity-10"
```

**Advanced Features:**
- **Flexible Layout**: Full width/height with flex centering
- **Input Design**: Minimal border, focus states, semi-transparent background
- **Text Hierarchy**: Extra small text size for compact display
- **Accessibility**: Proper focus management and outline removal with custom focus styles

### Radar Component Styling Architecture

#### 1. Container Responsive Design
```rust
// Main responsive container
class: "flex flex-col md:flex-row justify-center items-center md:items-start gap-4"
```

**Responsive Behavior:**
- **Mobile** (`flex-col`): Vertical stacking, center-aligned
- **Desktop** (`md:flex-row`): Horizontal layout, flex-start aligned
- **Spacing**: Consistent 16px gap between elements

#### 2. Component Sizing Strategy
```rust
// Graph container - fixed size
class: "flex-shrink-0"

// Legend container - flexible with padding
class: "p-4 flex-shrink-0"
```

**Design Philosophy:**
- **Graph**: Non-shrinking to maintain aspect ratio
- **Legend**: Padded container that doesn't shrink but allows content flexibility

#### 3. Legend Card Design
```rust
// Card container
class: "bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md"

// Title styling
class: "text-lg font-semibold mb-2"

// Content area
class: "space-y-2"
```

**Card System:**
- **Surface**: White background with dark mode override
- **Elevation**: Medium shadow for depth (`shadow-md`)
- **Spacing**: Internal padding and vertical rhythm
- **Typography**: Clear hierarchy with semibold headings

### SVG-Specific Styling Patterns

#### 1. Grid Component Dark Mode
```rust
// Grid lines
class: "dark:stroke-gray-600"

// Grid labels  
class: "fill-gray-600 dark:fill-gray-300 font-medium dark:font-semibold"
```

#### 2. Axis Component Styling
```rust
// Axis lines
class: "dark:stroke-gray-500"

// Axis labels
class: "dark:fill-gray-200"
```

#### 3. Legend Interactive States
```rust
// Dynamic state-based styling
let (rect_classes, text_classes) = if is_visible {
    ("", "")
} else {
    ("opacity-30", "line-through opacity-70")
};
```

**State Management:**
- **Visible**: Full opacity, normal text
- **Hidden**: 30% opacity, strikethrough text with 70% opacity

### Custom CSS Integration Points

#### 1. Tooltip State Management
```css
/* Pinned tooltip visual indicator */
.tooltip-pinned rect {
    stroke: #ffffff;
    stroke-width: 2px;
}

.dark .tooltip-pinned rect {
    stroke: #dddddd;
}
```

#### 2. SVG Interaction Layers
```css
/* Pointer event management for complex SVG interactions */
.tooltip { pointer-events: none; }
.radar-curve { pointer-events: visible; }
.data-point { pointer-events: visible; }
.data-point-circle { pointer-events: none; }
.data-point-hitarea { pointer-events: all; }
```

#### 3. Form Input Customizations
```css
/* Remove number input spinners in tooltips */
.editable-tooltip input[type=number]::-webkit-outer-spin-button,
.editable-tooltip input[type=number]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}
```

### Utility Usage Patterns

#### 1. Flexbox Patterns
- `flex`: Enable flexbox layout
- `flex-col` / `flex-row`: Direction control
- `items-center` / `items-start`: Cross-axis alignment
- `justify-center` / `justify-between`: Main-axis distribution
- `gap-2` / `gap-4`: Consistent spacing between flex items

#### 2. Sizing Utilities
- `w-full` / `h-full`: Full width/height
- `min-w-0`: Allow flex items to shrink below content size
- `flex-1`: Grow to fill available space
- `flex-shrink-0`: Prevent shrinking

#### 3. Spacing System
- `p-2` / `p-4`: Consistent padding (8px/16px)
- `px-1` / `px-2` / `px-4`: Horizontal padding variants
- `mb-2` / `mb-6`: Bottom margins (8px/24px)
- `space-y-2`: Vertical spacing between children

#### 4. Visual Effects
- `rounded` / `rounded-md` / `rounded-lg`: Border radius variants
- `shadow-md`: Medium drop shadow
- `opacity-30` / `opacity-70`: Transparency levels
- `transition-colors` / `duration-300`: Smooth animations

### Component-Specific Design Guidelines

#### 1. Interactive Elements
- Always include hover states with dark mode variants
- Use `transition-colors` for smooth interactions
- Consistent padding and border radius across similar elements

#### 2. Typography
- `text-xs` for compact UI elements (tooltips, small labels)
- `text-lg font-semibold` for section headings
- `font-medium` / `font-semibold` for emphasis without bold weight

#### 3. Dark Mode Implementation
- Every colored element should have a dark mode variant
- SVG elements use `fill` and `stroke` dark mode utilities
- Consistent dark color palette (gray-200, gray-300, gray-500, gray-600, gray-800)

#### 4. Responsive Design
- Use `md:` breakpoint for primary responsive changes
- Mobile-first approach with progressive enhancement
- Container patterns ensure consistent behavior across screen sizes