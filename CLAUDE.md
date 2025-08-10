# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains a Dioxus-based web application for creating radar graphs. The project is set up using Rust and the Dioxus framework, which is a React-like framework for building user interfaces in Rust. The application is primarily targeted for web deployment but can be built for desktop and mobile platforms as well.

## Development Environment

The project uses Nix Flakes for dependency management and development environment setup. The flake.nix file defines all the necessary dependencies and tools for the development environment.

## Build and Development Commands

### Setup and Installation

1. **Enter the development environment**:
   ```bash
   nix develop
   ```
   This will set up the environment with all required dependencies including Rust, dioxus-cli, and wasm-bindgen-cli.

2. **Run Tailwind CSS**:
   ```bash
   # When using Nix environment:
   tailwindcss -i ./tailwind.css -o ./assets/tailwind.css
   
   # If not using Nix:
   npm install
   npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css
   ```
   
   Note: For development, you might  request the user to use the command with the `--watch` flag in a separate terminal. But it should never be run by Claude.

### Development Commands

1. **Serve the application for development (with hot reloading)**:
   ```bash
   dx serve
   ```

2. **Build the application**:
   ```bash
   dx build
   ```

3. **Build for a specific platform**:
   ```bash
   # For web (default)
   dx build --features web
   
   # For desktop
   dx build --features desktop
   
   # For mobile
   dx build --features mobile
   ```

4. **Run the application for a different platform**:
   ```bash
   dx serve --platform desktop
   ```

### Tailwind CSS Development

When making UI changes, you can compile the CSS once without watching for changes:
```bash
# When using Nix environment:
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css

# If not using Nix:
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css
```

**IMPORTANT NOTE**: Never use the `--watch` parameter with Tailwind commands and never run `dx serve` when using Claude. These commands are run in separate terminal sessions.

## Project Architecture

### Core Structure

- `src/main.rs`: Main entry point for the application
- `src/components/`: Directory containing reusable UI components
  - `mod.rs`: Exports components
  - `hero.rs`: Hero component for the homepage
- `assets/`: Directory for static assets like images, CSS, etc.

### Key Concepts

1. **Component System**: The application follows a component-based architecture similar to React. Components are Rust functions annotated with `#[component]` that return an `Element`.

2. **RSX Macro**: UI elements are created using the `rsx!` macro, which is similar to JSX in React.

3. **Asset Management**: Static assets are referenced using the `asset!` macro, which handles bundling and path resolution.

4. **Feature Flags**: The application uses Cargo feature flags to enable different platform targets (web, desktop, mobile).

5. **Nix Integration**: The project uses a comprehensive Nix flake for dependency management, ensuring consistent development environments.

### Special Notes

- The wasm-bindgen version in Cargo.toml must match the wasm-bindgen-cli version provided by the Nix flake. Currently using version 0.2.100. When updating the Nix flake's wasm-bindgen-cli version, ensure you also update the wasm-bindgen dependency in Cargo.toml to match.

- The `flake.nix` file uses nixpkgs versions of both dioxus-cli and wasm-bindgen-cli for better maintainability.

### Rust Code Style and Structure

#### Module Organization

1. **File Structure**:
   - Use `<module>.rs` files instead of `<module>/mod.rs` files for module organization
   - Example: Prefer `src/components/radar/axis.rs` over `src/components/radar/axis/mod.rs`

2. **Module Hierarchy**:
   - Structure modules to reflect component relationships and usage patterns
   - Place subcomponents under their parent component's module
   - Example: `axis`, `curve`, and `grid` components are submodules of `radar` since they're only used within the radar component

3. **Imports and Exports**:
   - Use `self::` for clarity when importing from submodules
   - Re-export components at the appropriate level based on their usage

### Claude Code Instructions

- When working with Claude Code, DO NOT use `--watch` parameter for Tailwind or run `dx serve` commands. These commands are run in separate terminal sessions.
- For checking build errors, use `dx build` or `dx bundle` instead of `dx serve`.
- For applying Tailwind changes, run the command without the watch flag (`tailwindcss -i ./tailwind.css -o ./assets/tailwind.css`).
- Run all commands that rely on tools installed via nix with `nix develop --command ` to ensure the latest version of flake.nix is used
- **MANDATORY BEFORE COMMITTING**: Always run these commands before committing any code changes:
  ```bash
  # Format all Rust code
  nix develop --command cargo fmt
  
  # Check for clippy issues using - than evaluate and fix them
  nix develop --command cargo clippy
  ```
  These commands are REQUIRED and must be run before any commit. Never commit code without formatting and fixing all clippy issues.

## Agent-Specific Documentation System

This project includes comprehensive documentation designed for different types of AI agents and specialists. This documentation system helps future agent executions understand the codebase structure, patterns, and domain-specific knowledge.

### Documentation Schema

#### 1. Root-Level Specialized Documentation
- **`CLAUDE-RUST-EXPERT.md`**: Rust architecture, component patterns, state management, and advanced Rust techniques
- **`CLAUDE-TAILWIND-CSS.md`**: Styling system, dark mode implementation, responsive design patterns, and CSS architecture
- **`CLAUDE-NIX-FLAKE.md`**: Development environment, build system, dependency management, and Nix configuration
- **`CLAUDE-GENERAL.md`**: Project overview, features, user experience, and overall architecture

#### 2. Directory-Specific Documentation
Each relevant directory contains agent-specific documentation with the same naming pattern:
- **`src/CLAUDE-<AGENT-NAME>.md`**: Source-level patterns and organization
- **`src/components/CLAUDE-<AGENT-NAME>.md`**: Component-specific patterns and architecture
- **`src/components/radar/CLAUDE-<AGENT-NAME>.md`**: Feature-specific deep dives
- **`assets/CLAUDE-<AGENT-NAME>.md`**: Asset management and resource organization

#### 3. Agent Specialization Areas

**Rust Expert Agent:**
- Component architecture and hierarchy
- Signal-based state management patterns  
- Error handling and validation strategies
- Memory management and performance optimizations
- Platform abstraction techniques

**Tailwind CSS Specialist:**
- Utility-first styling approach
- Dark mode implementation strategies
- Responsive design patterns
- Custom CSS integration points
- Performance optimization techniques

**Nix Flake Architect:**
- Development environment configuration
- Cross-platform dependency management
- Build system integration
- Version synchronization strategies
- Performance and caching optimizations

**General Purpose Agent:**
- Overall project structure and workflows
- User experience design patterns
- Feature documentation and usage
- Integration patterns and best practices
- Maintenance and evolution guidelines

### Using This Documentation

#### For Future Agent Interactions
1. **Domain-Specific Tasks**: Read the relevant `CLAUDE-<AGENT-NAME>.md` files for specialized knowledge
2. **Component Work**: Check both root and directory-specific documentation for context
3. **Cross-Domain Tasks**: Review multiple agent documentation files for comprehensive understanding
4. **Architecture Decisions**: Reference the appropriate specialized documentation for design patterns

#### For Human Developers
1. **Onboarding**: Start with `CLAUDE-GENERAL.md` for project overview
2. **Specific Domains**: Deep dive into specialized documentation for detailed patterns
3. **Component Development**: Use directory-specific files for local context and patterns
4. **Maintenance**: Reference documentation when making changes to understand impact

#### Documentation Maintenance
- Update documentation when making significant architectural changes
- Keep specialized documentation synchronized with code changes
- Add new agent-specific files when introducing new domains or technologies
- Maintain consistency in documentation structure and depth across all agent files

This documentation system ensures that future AI agents have comprehensive, specialized knowledge to work effectively within this codebase while maintaining consistency with established patterns and practices.
