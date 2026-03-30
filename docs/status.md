# Status — 2026-03-30

## Build Status

| Platform | CI | Local |
|----------|-----|-------|
| Web (wasm32) | ✅ Passing | ✅ |
| Android aarch64 | 🔄 In progress | ✅ Built, deployed |
| Android x86_64 | 🔄 In progress | — |

## What's Working

- Dioxus 0.7.4 WASM frontend on 8 platforms
- Android APK (13MB, arm64) at https://solana.solfunmeme.com/onboarding/solfunmeme-dioxus.apk
- Embedded axum server on :8080 (status, pastebin, zkperf, stego, IPFS, forge)
- libp2p P2P (mDNS + gossipsub)
- erdfa-publish stego (ZWC text encoding)
- IPFS content addressing (erdfa-publish, rust-ipfs vendored)
- IPFS publish gate with ACL tiers + Solana ed25519 signing
- Generic forge client (Forgejo, Gitea, GitLab, Gogs)
- Nix flake: Android SDK/NDK 26.3, Rust cross-compile, JDK17
- CI: meta-introspector action forks, nix develop, cargo/nix caching

## Recent Changes

- Cleaned 14.5G (targets, temps, node_modules)
- Added bootstrap submodule (emojistage module)
- Merged Gödel 8D prime exponent projection
- Native deps behind `native` feature flag (no openssl in wasm)
- dioxus/mobile behind `native` feature (no desktop deps in web)
- Orphan submodules removed + documented
- Onboarding guide for new contributors

## Next

- Get all 3 CI platforms green
- Podping git notifications via libp2p gossipsub
- Forgejo webhook integration
- Start9 / Radicle forge support
- Brian's IPFS data sharing workflow
