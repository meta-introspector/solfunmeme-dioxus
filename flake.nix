{
  description = "solfunmeme-dioxus — WASM frontend with Solana wallet + DAO governance";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config openssl
            dioxus-cli
            binaryen
            chromium chromedriver
            nodePackages.tailwindcss
            nodejs_22
            python3
          ];

          shellHook = ''
            echo "🦀 solfunmeme-dioxus dev shell"
            echo "  make build         — dx build --release"
            echo "  make serve         — dx serve (dev)"
            echo "  make test-headless — headless browser tests"
            export CHROME_BIN="${pkgs.chromium}/bin/chromium"
            export CHROMEDRIVER="${pkgs.chromedriver}/bin/chromedriver"
          '';
        };
      }
    );
}
