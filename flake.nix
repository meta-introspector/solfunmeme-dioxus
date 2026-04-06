{
  description = "solfunmeme-dioxus — WASM frontend with Solana wallet + DAO governance";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    dioxus-src = {
      url = "github:meta-introspector/dioxus";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, dioxus-src }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
          config.allowUnfree = true;
          config.android_sdk.accept_license = true;
        };

        androidSdk = pkgs.androidenv.composeAndroidPackages {
          platformVersions = [ "33" "34" ];
          buildToolsVersions = [ "34.0.0" ];
          ndkVersions = [ "26.3.11579264" ];
          includeNDK = true;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [
            "wasm32-unknown-unknown"
            "aarch64-linux-android"
            "armv7-linux-androideabi"
            "x86_64-linux-android"
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        dx = craneLib.buildPackage {
          src = dioxus-src;
          pname = "dioxus-cli";
          version = "0.7.4";
          cargoExtraArgs = "-p dioxus-cli";
          strictDeps = true;
          buildInputs = with pkgs; [ openssl pkg-config cacert ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
          doCheck = false;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            dx
            pkg-config openssl
            binaryen
            chromium chromedriver
            nodePackages.tailwindcss
            nodejs_22
            python3
            androidSdk.androidsdk
            jdk17
          ];

          shellHook = ''
            # Gradle needs a writable SDK — copy from nix store on first use
            if [ ! -d "$HOME/.android-sdk" ]; then
              echo "📋 Creating writable Android SDK copy..."
              cp -rL ${androidSdk.androidsdk}/libexec/android-sdk $HOME/.android-sdk
              chmod -R u+w $HOME/.android-sdk
            fi
            export ANDROID_HOME="$HOME/.android-sdk"
            export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/26.3.11579264"
            export CHROME_BIN="${pkgs.chromium}/bin/chromium"
            export CHROMEDRIVER="${pkgs.chromedriver}/bin/chromedriver"
            echo "🦀 solfunmeme-dioxus dev shell"
            echo "  make build         — dx build --release"
            echo "  make serve         — dx serve (dev)"
            echo "  make android       — dx build --platform android --release"
            echo "  make test-headless — headless browser tests"
            echo "🤖 Android SDK: $ANDROID_HOME"
            echo "🔧 NDK: $ANDROID_NDK_HOME"
          '';
        };
      }
    );
}
