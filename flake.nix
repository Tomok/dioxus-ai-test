{
  description = "Dioxus development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain with wasm32-unknown-unknown target
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Crane for Rust builds
        craneLib = crane.mkLib pkgs;

        # System-specific dependencies
        systemDeps = with pkgs; [
          # Build essentials
          pkg-config
          openssl
          cacert
          cmake
          
          # For web development
          wasm-pack
          binaryen
          
          # For Linux desktop apps
          gtk3
          webkitgtk_4_1
          libappindicator-gtk3
          
          # For audio/video (optional)
          alsa-lib
          libpulseaudio
          
          # Build tools
          gcc
          libiconv
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          # macOS-specific dependencies
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.CoreServices
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          pkgs.darwin.apple_sdk.frameworks.WebKit
          pkgs.darwin.apple_sdk.frameworks.Cocoa
        ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
          # Linux-specific dependencies
          libxkbcommon
          libGL
          fontconfig
          freetype
          expat
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          vulkan-loader
        ];

        # Dioxus CLI via cargo install, without tests
        dioxus-cli = pkgs.rustPlatform.buildRustPackage {
          pname = "dioxus-cli";
          version = "0.6.0";
          
          src = pkgs.fetchCrate {
            pname = "dioxus-cli";
            version = "0.6.0";
            sha256 = "sha256-0Kg2/+S8EuMYZQaK4Ao+mbS7K48VhVWjPL+LnoVJMSw=";
          };
          
          cargoHash = "sha256-uD3AHHY3edpqyQ8gnsTtxQsen8UzyVIbArSvpMa+B+8=";
          
          # Skip tests completely
          doCheck = false;
          
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
          ];

          buildInputs = systemDeps;

          # Environment variables for the build
          OPENSSL_NO_VENDOR = "1";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          
          # Add CA certificates path
          SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
          NIX_SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
        };


      in
      {
        packages = {
          dioxus-cli = dioxus-cli;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain with wasm32-unknown-unknown target
            rustToolchain
            
            # Dioxus CLI (try crane build first, then fallback)
            dioxus-cli
            
            # Cargo tools
            cargo-watch
            cargo-edit
            
            # Development tools
            nodejs_20
            nodePackages.npm
            
            # System dependencies
          ] ++ systemDeps;

          shellHook = ''
            echo "🦀 Dioxus Development Environment"
            echo "==============================="
            echo ""
            echo "Available commands:"
            echo "  dx new <n> - Create a new Dioxus project"
            echo "  dx serve      - Serve the project with hot reloading"
            echo "  dx build      - Build the project"
            echo "  dx --help     - Show all available commands"
            echo ""
            
            # Check if dx is available
            if command -v dx &> /dev/null; then
              echo "✅ dioxus-cli is available: $(dx --version)"
            else
              echo "⚠️  dioxus-cli not found in PATH"
            fi
            echo ""
            
            # Set up environment variables
            export RUST_LOG=info
            export CARGO_TARGET_DIR="$PWD/target"
            
            # OpenSSL for Rust
            export OPENSSL_NO_VENDOR=1
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
            
            # CA certificates
            export SSL_CERT_FILE="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            export NIX_SSL_CERT_FILE="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            
            echo "To create your hot_dog project:"
            echo "  dx new hot_dog"
            echo "  cd hot_dog"
            echo "  dx serve"
            echo ""
          '';

          # Environment variables for Rust
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          OPENSSL_NO_VENDOR = "1";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          
          # Add CA certificates path
          SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
          NIX_SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
        };
      }
    );
}