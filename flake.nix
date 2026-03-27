{
  description = "Solfunmeme-Dioxus: Self-Aware Codebase";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    android-nixpkgs = {
      url = "github:nickcao/nix-android";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, crane, android-nixpkgs }:
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
            platformVersions = [ "34" ];
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
            ANDROID_HOME = "${androidSdk.androidsdk}/libexec/android-sdk";
            ANDROID_NDK_HOME = "${androidSdk.androidsdk}/libexec/android-sdk/ndk/26.3.11579264";

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
