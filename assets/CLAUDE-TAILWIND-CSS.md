# CLAUDE-TAILWIND-CSS.md - Assets & Styling Resources

## Asset Management & Styling Architecture

### CSS Asset Organization

#### 1. File Structure
```
assets/
├── favicon.ico                 # Application icon
├── header.svg                 # Header graphic asset
├── styling/
│   └── main.css              # Custom base styles & legacy compatibility
└── tailwind.css              # Compiled Tailwind utilities + custom additions
```

#### 2. Asset Loading Strategy
**In main.rs:**
```rust
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Loading order is important
document::Link { rel: "stylesheet", href: MAIN_CSS }
document::Link { rel: "stylesheet", href: TAILWIND_CSS }
```

**Loading Priority:**
1. **main.css**: Base styles, theme foundation, legacy compatibility
2. **tailwind.css**: Utility classes, component styles, custom extensions

### Custom CSS Architecture (`main.css`)

#### 1. Theme Foundation
```css
/* Light mode (default) */
body {
    background-color: #ffffff;
    color: #333333;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    transition: background-color 0.3s, color 0.3s;
}

/* Dark mode - applied when .dark class is present */
.dark body {
    background-color: #0f1116;
    color: #ffffff;
}
```

**Design Principles:**
- System font stack for performance and consistency
- Smooth transitions for theme switching
- Custom dark background color (`#0f1116`) for better contrast

#### 2. Legacy Component Styles
```css
/* Existing styles for non-Tailwind components */
#hero, #links, #header {
    /* Legacy layout and styling */
    /* Maintained for backward compatibility */
}
```

### Compiled Tailwind CSS (`tailwind.css`)

#### 1. Tailwind Base Styles
- CSS reset and normalization
- Custom property definitions for utilities
- Browser compatibility styles

#### 2. Container System
```css
.container {
    width: 100%;
}

@media (min-width: 640px) { .container { max-width: 640px; } }
@media (min-width: 768px) { .container { max-width: 768px; } }
@media (min-width: 1024px) { .container { max-width: 1024px; } }
@media (min-width: 1280px) { .container { max-width: 1280px; } }
@media (min-width: 1536px) { .container { max-width: 1536px; } }
```

#### 3. Custom Component Extensions
```css
/* Tooltip state management */
.tooltip-pinned rect {
    stroke: #ffffff;
    stroke-width: 2px;
}

.dark .tooltip-pinned rect {
    stroke: #dddddd;
}

/* SVG interaction layers */
.tooltip { pointer-events: none; }
.radar-curve { pointer-events: visible; }
.data-point { pointer-events: visible; }
.data-point-circle { pointer-events: none; }
.data-point-hitarea { pointer-events: all; }
```

#### 4. Form Input Customizations
```css
/* Remove spinner arrows from number inputs in tooltips */
.editable-tooltip input[type=number]::-webkit-outer-spin-button,
.editable-tooltip input[type=number]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

.editable-tooltip input[type=number] {
    -moz-appearance: textfield;
}
```

### Build Process Integration

#### 1. Source Configuration (`tailwind.css` source)
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Custom component styles */
@layer components {
    /* Component-specific styles would go here */
}

@layer utilities {
    /* Custom utility extensions would go here */
}
```

#### 2. Content Scanning
**From `tailwind.config.js`:**
```javascript
content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"]
```

**Scanning Strategy:**
- Scans all Rust source files for class usage
- Includes HTML and CSS files for comprehensive coverage
- Outputs only used utilities to minimize bundle size

#### 3. Build Commands
```bash
# Development build
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css

# Production build (with minification)
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --minify
```

### Performance Optimizations

#### 1. CSS Bundle Size Management
- Tailwind's purge process removes unused utilities
- Custom colors reduce utility bloat
- Component-level custom CSS minimized

#### 2. Loading Performance
- Single CSS file reduces HTTP requests
- Critical styles loaded before JavaScript
- Smooth theme transitions without FOUC (Flash of Unstyled Content)

#### 3. Runtime Performance
- Class-based theme switching (no style recalculation)
- CSS custom properties for dynamic values where needed
- Minimal specificity conflicts between custom and utility styles

### Development Workflow

#### 1. Style Development Process
1. **Modify**: Edit class attributes in Rust components
2. **Compile**: Run `tailwindcss -i ./tailwind.css -o ./assets/tailwind.css`
3. **Test**: Hot reload via `dx serve` (development only)
4. **Validate**: Check theme switching and responsive behavior

#### 2. Custom Style Integration
**For Tailwind-compatible styles:**
- Add to `tailwind.config.js` theme extension
- Use `@layer` directive in source CSS

**For SVG-specific or complex interactions:**
- Add to `assets/tailwind.css` after compilation
- Use specific selectors to avoid utility conflicts

#### 3. Asset Management Best Practices
- Keep custom CSS minimal (90% should be Tailwind utilities)
- Use semantic color names in config for maintainability
- Document custom classes and their purposes
- Version control both source and compiled CSS

### Browser Compatibility

#### 1. Modern Browser Features
- CSS custom properties for dynamic styling
- CSS Grid and Flexbox for layouts
- CSS transitions for smooth interactions

#### 2. Fallback Strategies
- System font stack ensures consistent typography
- Progressive enhancement for advanced features
- Graceful degradation for older browsers

### Maintenance Guidelines

#### 1. Regular Maintenance Tasks
- Update Tailwind CSS version for new utilities and bug fixes
- Audit custom CSS for consolidation opportunities
- Monitor bundle size for performance impact

#### 2. Code Organization
- Keep theme-related styles in main.css
- Keep component interactions in tailwind.css
- Document any non-obvious custom CSS patterns

#### 3. Integration Testing
- Test theme switching across all components
- Validate responsive behavior at all breakpoints
- Ensure SVG interactions work across browsers