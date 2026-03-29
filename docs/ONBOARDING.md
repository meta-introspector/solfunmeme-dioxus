# SOLFUNMEME — Getting Started Guide

Welcome! This guide gets you from zero to running a SOLFUNMEME P2P node on your phone or computer.

## What Is This?

SOLFUNMEME is a decentralized app that runs on your device as a **full node**. It serves a web interface, connects to other nodes via P2P, stores data on IPFS, and syncs with git forges. Think of it as your personal server that fits in your pocket.

## 1. Install the App

### Android (easiest)

1. Open this URL on your phone: **https://solana.solfunmeme.com/onboarding/solfunmeme-dioxus.apk**
2. Tap "Download", then tap the downloaded file to install
3. If prompted about "unknown sources", tap Settings → Allow from this source → Install
4. Open the app — it starts a local server on port 8080

### From Source (Linux/Mac)

```bash
# Clone
git clone https://github.com/meta-introspector/solfunmeme-dioxus
cd solfunmeme-dioxus

# Enter dev shell (installs Rust, Android SDK, everything)
nix develop

# Build for web
dx build --release --platform web

# Build for Android
dx build --features native --platform android --release --target aarch64-linux-android
```

## 2. Your Node Is Running

Once the app starts, you have a web server at `http://localhost:8080` with these services:

| What | URL | Try It |
|------|-----|--------|
| Status | http://localhost:8080/status | See your node info |
| Pastebin | http://localhost:8080/paste | Share text snippets |
| ZK Witness | http://localhost:8080/zkperf | Generate a proof witness |
| Stego Encode | http://localhost:8080/stego/encode | Hide data in zero-width characters |
| IPFS | http://localhost:8080/ipfs | Store content-addressed data |
| Git Repos | http://localhost:8080/forge/repos | Browse connected git forges |
| Peers | http://localhost:8080/peers | See P2P connections |

**Other devices on your WiFi can connect to your phone's IP on port 8080.**

## 3. Share Data with IPFS

### Add content
```bash
curl -X POST http://localhost:8080/ipfs -d "Hello from my node!"
# Returns: "QmXyz..."  (your content's unique ID)
```

### Fetch content
```bash
curl http://localhost:8080/ipfs/QmXyz...
# Returns: "Hello from my node!"
```

### Publish with security review
```bash
# Mark as public (anyone can read)
curl -X POST http://localhost:8080/ipfs/publish \
  -H "Content-Type: application/json" \
  -d '{"cid": "QmXyz...", "acl": "public"}'

# Mark as holder-only (token holders can read)
curl -X POST http://localhost:8080/ipfs/publish \
  -H "Content-Type: application/json" \
  -d '{"cid": "QmXyz...", "acl": "holder"}'
```

Every published item gets a **post-quantum signed header** (ML-DSA-44) with a Merkle root — verifiable by anyone.

## 4. Use the Pastebin

```bash
# Create a paste
curl -X POST http://localhost:8080/paste \
  -H "Content-Type: application/json" \
  -d '{"id":"","content":"my first paste","timestamp":0}'

# Fetch it back (use the ID from the response)
curl http://localhost:8080/paste/abc123def
```

## 5. Steganography (Hide Data in Text)

Encode secret data as invisible zero-width characters:

```bash
# Encode
curl -X POST http://localhost:8080/stego/encode \
  -H "Content-Type: application/json" \
  -d '{"data": "secret message"}'

# Decode
curl -X POST http://localhost:8080/stego/decode \
  -d '<paste the carrier text here>'
```

The encoded text looks empty but contains your data!

## 6. Connect to a Git Forge

If you run Forgejo, Gitea, or GitLab locally:

```bash
# Set your token
export FORGEJO_TOKEN=your_token_here

# Start the app — it auto-connects to localhost:3000
# Browse repos at:
curl http://localhost:8080/forge/repos
```

## 7. Connect to Other Nodes

Your node automatically discovers other SOLFUNMEME nodes on the local network via **mDNS**. For internet connections, nodes use **libp2p gossipsub** to share updates.

Check connected peers:
```bash
curl http://localhost:8080/peers
```

## 8. The Math (Optional Fun)

Every piece of data gets coordinates in the **Monster Group** — the largest sporadic simple group with 196,883 dimensions:

- **Crown product**: 47 × 59 × 71 = 196,883
- **Orbifold coordinates**: (data mod 71, data mod 59, data mod 47)
- **ZK witnesses** include these coordinates for mathematical provenance

```bash
# Generate a witness
curl http://localhost:8080/zkperf
# Returns: timestamp, commitment hash, orbifold coords, crown product
```

## Quick Reference

| Task | Command |
|------|---------|
| Check node | `curl localhost:8080/status` |
| Add to IPFS | `curl -X POST localhost:8080/ipfs -d "data"` |
| Create paste | `curl -X POST localhost:8080/paste -H 'Content-Type: application/json' -d '{"id":"","content":"hi","timestamp":0}'` |
| Encode stego | `curl -X POST localhost:8080/stego/encode -H 'Content-Type: application/json' -d '{"data":"secret"}'` |
| List repos | `curl localhost:8080/forge/repos` |
| ZK witness | `curl localhost:8080/zkperf` |
| List peers | `curl localhost:8080/peers` |

## Need Help?

- **Source**: https://github.com/meta-introspector/solfunmeme-dioxus
- **Docs**: https://solana.solfunmeme.com/dioxus/
- **Architecture**: See `docs/FORGE_SYNC.md` in the repo
- **Token**: BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump

Welcome to the mesh! 🌐
