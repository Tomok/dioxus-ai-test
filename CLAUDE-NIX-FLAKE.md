# CLAUDE-NIX-FLAKE.md

## Dioxus Radar Graph Application - Nix Flake Development Environment

### Overview
This project uses Nix Flakes to provide a reproducible, cross-platform development environment for Dioxus application development. The flake architecture ensures consistent tool versions, proper dependency management, and platform-specific optimizations across different operating systems.

### Flake Architecture

#### 1. Input Dependencies
```nix
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  flake-utils.url = "github:numtide/flake-utils";
  rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  crane = {
    url = "github:ipetkov/crane";
  };
};
```

**Dependency Strategy:**
- **nixpkgs**: Uses unstable branch for latest packages and security updates
- **flake-utils**: Provides cross-platform build utilities
- **rust-overlay**: Enables precise Rust toolchain version control with WASM targets
- **crane**: Advanced Rust build system integration (prepared for future use)

#### 2. Lock File Analysis (`flake.lock`)
**Current Locked Versions:**
- **nixpkgs**: `62e0f05ede1da0d54515d4ea8ce9c733f12d9f08` (recent unstable)
- **rust-overlay**: `8668ca94858206ac3db0860a9dec471de0d995f8` (latest)
- **crane**: `471f8cd756349f4e86784ea10fdc9ccb91711fca` (build system ready)
- **flake-utils**: `11707dc2f618dd54ca8739b309ec4fc024de578b` (stable)

### Rust Toolchain Management

#### 1. Toolchain Configuration
```nix
rustToolchain = pkgs.rust-bin.stable.latest.default.override {
  targets = [ "wasm32-unknown-unknown" ];
};
```

**Features:**
- **Latest Stable**: Always uses the most recent stable Rust release
- **WASM Target**: Includes WebAssembly compilation support
- **Cross-Compilation**: Ready for multi-target builds

#### 2. Version Synchronization Strategy
**Critical Version Alignment:**
```nix
# In flake.nix - wasm-bindgen-cli from nixpkgs
wasm-bindgen-cli

# In Cargo.toml - must match
wasm-bindgen = "=0.2.97"
```

**Current Versions:**
- **wasm-bindgen**: `0.2.97` (exact version match required)
- **dioxus**: `0.6.0` (latest stable)

### Custom Dioxus CLI Build

#### 1. Source-Based Installation
```nix
dioxus-cli = pkgs.rustPlatform.buildRustPackage {
  pname = "dioxus-cli";
  version = "0.6.0";
  
  src = pkgs.fetchCrate {
    pname = "dioxus-cli";
    version = "0.6.0";
    sha256 = "sha256-0Kg2/+S8EuMYZQaK4Ao+mbS7K48VhVWjPL+LnoVJMSw=";
  };
  
  cargoHash = "sha256-uD3AHHY3edpqyQ8gnsTtxQsen8UzyVIbArSvpMa+B+8=";
  doCheck = false;  # Skip tests for faster builds
};
```

**Benefits:**
- **Version Control**: Exact version matching with project dependencies
- **Build Optimization**: Tests disabled for faster CI/development
- **Reproducibility**: Same binary across all development environments

#### 2. Build Environment Configuration
```nix
nativeBuildInputs = with pkgs; [
  pkg-config
  rustToolchain
];

buildInputs = systemDeps;

# SSL/TLS Configuration
OPENSSL_NO_VENDOR = "1";
OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
```

### Platform-Specific Dependencies

#### 1. Cross-Platform Strategy
```nix
systemDeps = with pkgs; [
  # Universal build tools
  pkg-config openssl cacert cmake
  
  # Web development
  wasm-pack binaryen
  
  # Desktop (Linux/macOS common)
  gtk3 webkitgtk_4_1 libappindicator-gtk3
  
  # Audio/Video support
  alsa-lib libpulseaudio
] ++ lib.optionals stdenv.isDarwin [ /* macOS deps */ ]
  ++ lib.optionals stdenv.isLinux [ /* Linux deps */ ];
```

#### 2. macOS-Specific Dependencies
```nix
pkgs.darwin.apple_sdk.frameworks.Security
pkgs.darwin.apple_sdk.frameworks.CoreServices  
pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
pkgs.darwin.apple_sdk.frameworks.WebKit
pkgs.darwin.apple_sdk.frameworks.Cocoa
```

#### 3. Linux-Specific Dependencies
```nix
libxkbcommon libGL fontconfig freetype expat
xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi
vulkan-loader xdotool  # For desktop builds
```

### Development Shell Configuration

#### 1. Tool Suite
```nix
buildInputs = with pkgs; [
  # Rust ecosystem
  rustToolchain
  dioxus-cli
  wasm-bindgen-cli
  
  # Cargo utilities
  cargo-watch
  cargo-edit
  
  # Node.js ecosystem for Tailwind
  nodejs_20
  nodePackages.npm
  nodePackages.tailwindcss
  
  # Claude Code CLI
  claude-code
] ++ systemDeps;
```

#### 2. Environment Variables
```nix
# Rust configuration
RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
RUST_LOG = "info";
CARGO_TARGET_DIR = "$PWD/target";

# OpenSSL configuration
OPENSSL_NO_VENDOR = "1";
OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

# SSL certificates
SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
NIX_SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
```

#### 3. Shell Hook Information
```nix
shellHook = ''
  echo "🦀 Dioxus Development Environment"
  echo "Available commands:"
  echo "  dx serve      - Hot reloading development server"
  echo "  dx build      - Production build"
  echo "  tailwindcss   - CSS compilation"
  echo "  claude        - AI coding assistant"
'';
```

### Build System Integration

#### 1. Asset Compilation Strategy
**Tailwind CSS Integration:**
- **Source**: `tailwind.css` (utility directives)
- **Output**: `assets/tailwind.css` (compiled)
- **Tool**: `nodePackages.tailwindcss` from nixpkgs

**Alternative Build Methods:**
- **Nix Command**: `nix develop --command tailwindcss -i ./tailwind.css -o ./assets/tailwind.css`
- **NPM Script**: `npm run build:tailwind` (using package.json)

#### 2. Development Commands
```bash
# Enter development environment
nix develop

# Start development server (NOT for Claude usage)
dx serve

# Build application
dx build

# Compile Tailwind CSS
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css

# Format and lint code (MANDATORY before commits)
cargo fmt
cargo clippy
```

### Package Management Strategy

#### 1. Hybrid Approach
**Nix-Managed:**
- Rust toolchain and system dependencies
- Dioxus CLI and wasm-bindgen-cli
- System libraries and build tools

**NPM-Managed:**
```json
{
  "devDependencies": {
    "tailwindcss": "^3.3.5"
  },
  "scripts": {
    "tailwind": "tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch",
    "build:tailwind": "tailwindcss -i ./tailwind.css -o ./assets/tailwind.css"
  }
}
```

#### 2. Version Synchronization
**Critical Alignment Points:**
- `wasm-bindgen` crate version must match `wasm-bindgen-cli` version
- Tailwind CSS version should match between Nix and NPM (for consistency)
- Dioxus CLI version should match Dioxus library version

### Performance Optimizations

#### 1. Build Speed Enhancements
```nix
# Skip tests in dioxus-cli build
doCheck = false;

# Optimized target directory
CARGO_TARGET_DIR = "$PWD/target";

# Parallel builds enabled by default
```

#### 2. Dependency Caching
- Nix store provides automatic dependency caching
- Cargo dependencies cached per-project
- WebAssembly builds cached across development sessions

### Security & Certificate Management

#### 1. SSL/TLS Configuration
```nix
# System OpenSSL integration
OPENSSL_NO_VENDOR = "1";
OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

# Certificate authority bundle
SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
```

#### 2. Unfree Package Permissions
```nix
config.allowUnfreePredicate = pkg: builtins.elem (pkgs.lib.getName pkg) [
  "claude-code"  # Allow Claude Code CLI
];
```

### Development Workflow Integration

#### 1. Standard Development Commands
```bash
# Initial setup
nix develop

# Development cycle
dx build                    # Check compilation
cargo fmt                  # Format code
cargo clippy               # Lint code
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css  # Update styles
```

#### 2. Claude Code Integration
**Environment Requirements:**
- Must use `nix develop --command` for tools requiring dev environment
- Temporary tools via `nix run nixpkgs#<tool>` when needed
- All Rust tools (cargo, rustc) must run through Nix environment

#### 3. Git Hooks Integration
- Pre-commit hooks ensure code formatting and linting
- Automated Tailwind CSS compilation in CI/CD
- Version consistency checks across Cargo.toml and flake.nix

### Maintenance & Updates

#### 1. Regular Update Process
```bash
# Update flake inputs
nix flake update

# Update NPM dependencies  
npm update

# Sync version numbers between Cargo.toml and flake.nix
```

#### 2. Version Compatibility Matrix
| Component | Version | Compatibility Notes |
|-----------|---------|-------------------|
| Rust | Latest Stable | Via rust-overlay |
| Dioxus | 0.6.0 | CLI and library must match |
| wasm-bindgen | 0.2.97 | Exact match required |
| Tailwind | ^3.3.5 | Latest compatible |

#### 3. Platform Testing Strategy
- Linux: Primary development and CI platform
- macOS: Secondary development platform with framework dependencies
- Windows: Via WSL2 or dedicated Windows Nix installation

### Troubleshooting Guide

#### 1. Common Issues
**Version Mismatch:**
- Check wasm-bindgen versions in Cargo.toml vs flake.nix
- Verify Dioxus CLI and library versions match

**SSL Certificate Issues:**
- Ensure `SSL_CERT_FILE` environment variable is set
- Verify cacert package is available in development shell

**Build Failures:**
- Check that all system dependencies are available
- Verify OpenSSL configuration for Rust builds

#### 2. Platform-Specific Issues
**macOS:**
- Ensure Xcode Command Line Tools are installed
- Verify macOS SDK frameworks are accessible

**Linux:**
- Check that graphics libraries are installed (libGL, Vulkan)
- Ensure X11 development packages are available

### Future Enhancements

#### 1. Planned Improvements
- **Crane Integration**: Full Rust build system optimization
- **Devbox Integration**: Alternative to traditional Nix development shells  
- **CI/CD Templates**: Pre-configured GitHub Actions with Nix caching

#### 2. Scalability Considerations
- **Multi-Target Builds**: Extended platform support (mobile, embedded)
- **Containerization**: Docker integration with Nix-generated images
- **Distribution**: AppImage and flatpak generation via Nix