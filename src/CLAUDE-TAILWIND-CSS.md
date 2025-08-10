# CLAUDE-TAILWIND-CSS.md - Source Styling Architecture

## Source Code Styling Patterns

### Main Application Styling (`main.rs`)

#### 1. Root Container Design
```rust
// Main application wrapper
class: "container mx-auto px-4 py-8 bg-background dark:bg-gray-900 text-text dark:text-white min-h-screen transition-colors duration-300"
```

**Design Analysis:**
- `container mx-auto`: Responsive centered container with max-widths
- `px-4 py-8`: Consistent horizontal and vertical padding
- `bg-background dark:bg-gray-900`: Custom semantic color with dark override
- `text-text dark:text-white`: Semantic text color with dark mode support
- `min-h-screen`: Full viewport height minimum
- `transition-colors duration-300`: Smooth theme transitions (300ms)

#### 2. Header Layout Pattern
```rust
// App header with title and controls
class: "flex justify-between items-center mb-6"
```

**Features:**
- Flexbox layout with space distribution
- Center-aligned items vertically
- Consistent bottom margin (`mb-6` = 24px)

#### 3. Typography Hierarchy
```rust
// Main heading style
class: "text-3xl font-bold"
```

**Scale:**
- `text-3xl`: 30px font size with 36px line height
- `font-bold`: 700 weight for strong hierarchy

### Component Styling Patterns

#### 1. Theme Button Styling
```rust
// Interactive button pattern
class: "p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
```

**Interaction Design:**
- `p-2`: 8px padding on all sides
- `rounded-md`: 6px border radius
- Hover states with dark mode variants
- Smooth color transitions

#### 2. Theme Button Container
```rust
// Button group layout
class: "flex gap-2 items-center"
```

**Layout:**
- Horizontal flex layout
- 8px gap between buttons
- Vertically centered alignment

### Asset Integration Strategy

#### 1. CSS Asset Management
```rust
// Compile-time asset linking
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Document head integration
document::Link { rel: "stylesheet", href: MAIN_CSS }
document::Link { rel: "stylesheet", href: TAILWIND_CSS }
```

#### 2. Asset Loading Order
1. Main CSS (base styles and legacy compatibility)
2. Tailwind CSS (utility classes and custom additions)

### Platform-Specific Styling

#### 1. Web-Specific Features
- JavaScript-based theme switching
- LocalStorage persistence
- System preference detection
- DOM class manipulation

#### 2. Non-Web Fallbacks
- Theme buttons render as empty elements
- No dynamic theme switching capability
- Static styling only

### Responsive Design Implementation

#### 1. Container Responsiveness
```rust
// Responsive radar container
class: "container mx-auto"
```

**Breakpoint Behavior:**
- Mobile: Full width with padding
- Tablet (768px+): Max width 768px, centered
- Desktop (1024px+): Max width 1024px, centered
- Large (1280px+): Max width 1280px, centered

### Theme System Architecture

#### 1. Theme Initialization Script
```javascript
// Embedded JavaScript for theme management
function applyTheme() {
    if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        document.documentElement.classList.add('dark');
    } else {
        document.documentElement.classList.remove('dark');
    }
}
```

#### 2. Theme Persistence Strategy
- `localStorage.theme = 'light'`: Force light mode
- `localStorage.theme = 'dark'`: Force dark mode
- `localStorage.removeItem('theme')`: Follow system preference

### Performance Considerations

#### 1. CSS Loading Strategy
- Minimal CSS files for faster loading
- Tailwind purging reduces bundle size
- Critical styles in main.css for immediate rendering

#### 2. Theme Switching Performance
- Class-based switching (no style recalculation)
- Transition animations for smooth UX
- Immediate theme application on page load

### Development Workflow

#### 1. Style Modification Process
1. Edit class attributes in Rust components
2. Recompile Tailwind CSS: `tailwindcss -i ./tailwind.css -o ./assets/tailwind.css`
3. Hot reload via `dx serve` (development only)
4. Test theme switching functionality

#### 2. Custom Style Integration
- Add custom CSS to `assets/styling/main.css` for legacy/special cases
- Add Tailwind extensions to `assets/tailwind.css` for framework integration
- Use utility classes for 90% of styling needs