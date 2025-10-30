{
  description = "Citrine - A Clojure-like language implemented in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Use a stable rust version
        rustVersion = pkgs.rust-bin.stable.latest.default;
        
        # Create a custom rust toolchain with additional components
        rustToolchain = rustVersion.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };
        
        # Development dependencies
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          
          # Development tools
          cargo-watch
          cargo-edit
          cargo-audit
          cargo-expand
          cargo-insta
          
          # Documentation
          mdbook
        ];
        
        # Runtime dependencies
        buildInputs = with pkgs; [
          # Add any runtime dependencies here
        ];
        
        # Environment variables
        env = {
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
        
      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          
          shellHook = ''
            echo "🦀 Welcome to Citrine development environment 🦀"
            echo "Rust toolchain: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
            
            # Set environment variables
            ${builtins.concatStringsSep "\n" (builtins.attrValues (builtins.mapAttrs (name: value: "export ${name}=${value}") env))}
          '';
        };
        
        # For future: add packages, apps, etc.
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "citrine";
            version = "0.1.0";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            nativeBuildInputs = [ rustToolchain ];
            buildInputs = buildInputs;
          };
        };
      }
    );
}

