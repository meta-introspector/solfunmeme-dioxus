{
  description = "Solfunmeme-Dioxus: Self-Aware Codebase";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, crane }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
            config.allowUnfree = true;
            config.android_sdk.accept_license = true;
          };
          rustToolchain = pkgs.rust-bin.stable.latest.default;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          commonArgs = {
            src = let
              rawSrc = craneLib.cleanCargoSource ./.;
            in pkgs.lib.cleanSourceWith {
              src = rawSrc;
              filter = path: type:
                ! (pkgs.lib.hasInfix "/src/bin/Cargo.toml" path
                  || pkgs.lib.hasInfix "/src/bin/Cargo.lock" path);
            };
            strictDeps = true;
            buildInputs = with pkgs; [
              openssl
              pkg-config
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];
            nativeBuildInputs = with pkgs; [ pkg-config ];
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        {
          default = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });
        }
      );

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
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
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustToolchain
              openssl
              pkg-config
              nodejs
              nodePackages.npm
              androidSdk.androidsdk
              jdk17
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
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
              export PATH="$HOME/.cargo/bin:$PATH"
              echo "🤖 Android SDK: $ANDROID_HOME"
              echo "🔧 NDK: $ANDROID_NDK_HOME"
              echo ""
              echo "Build: dx build --platform android --release"
              echo "   or: make android"
            '';
          };
        }
      );
    };
}
