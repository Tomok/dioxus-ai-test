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

2. **Install Tailwind CSS** (if not using Nix):
   ```bash
   npm install
   npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
   ```

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

When making UI changes, run the Tailwind CSS compiler to automatically update the CSS:
```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

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

- The project requires a specific version of wasm-bindgen (0.2.97) to ensure compatibility with the installed wasm-bindgen-cli. This is handled by the Nix flake but should be considered when modifying dependencies.

- The `flake.nix` file contains a custom build of wasm-bindgen-cli version 0.2.97 to ensure compatibility.

- All code should be formatted using `cargo fmt` before committing.