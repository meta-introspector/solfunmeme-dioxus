# P2P Git Forge Sync — Architecture

## Overview

The solfunmeme-dioxus app acts as a P2P node that can sync with any self-hosted git forge. Browsers connect to the phone, the phone connects to forges and IPFS.

```
Brian's IPFS node ──→ IPFS network ──→ Phone (erdfa-publish ipfs module)
                                         ↕
Local Forgejo :3000 ←──→ Phone :8080 ←──→ Browser
                                         ↕
                                    libp2p gossipsub
                                         ↕
                                    Other phones/nodes
```

## Supported Forges

| Forge | API | Status |
|-------|-----|--------|
| Forgejo | v1 (Gitea-compatible) | ✅ Implemented |
| Gitea | v1 | ✅ Implemented |
| GitLab | v4 | ✅ Implemented |
| Gogs | v1 (Gitea-compatible) | ✅ Implemented |
| GitHub | REST v3 / GraphQL v4 | Planned |
| Start9 | Embassy API | Planned |
| Radicle | P2P native | Planned |

## Node Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| /forge/repos | GET | List repos from all configured forges |
| /ipfs/{cid} | GET | Fetch content by CID (local flatfs or network) |
| /ipfs | POST | Add content to IPFS, return CID |

## Configuration

Forges are configured via environment variables:
```
FORGEJO_TOKEN=<token>   # local Forgejo at :3000
GITEA_TOKEN=<token>     # remote Gitea
GITLAB_TOKEN=<token>    # GitLab instance
```

## IPFS Integration (via erdfa-publish)

- Pure Rust IPFS: `rust-unixfs` FileAdder + flatfs block store
- DASL/CBOR envelopes with Monster Group orbifold coordinates
- Three backends: RustStore, DaslCborStore, IpfsCliStore
- Brian shares data → IPFS CID → phone fetches via `/ipfs/{cid}`

## Podping / Git Notifications (Planned)

- Subscribe to forge webhooks for push events
- Broadcast via libp2p gossipsub topic `solfunmeme/git-updates`
- Peers receive notifications and can pull changes
- Compatible with Podping protocol for podcast-style update feeds

## Future: zos-server Plugin

The forge client will be extracted as a zos-server plugin (`.so`):
```
zos/plugins/forge-sync.so
  - ForgePlugin::list_repos()
  - ForgePlugin::sync_repo(name)
  - ForgePlugin::webhook_handler()
```

Jocko fuzz testing will exercise the forge API client against all backends.

## References

- https://blog.yaakov.online/self-hosted-git-and-ci-cd-for-fun-and-profit/
- https://gitlab.com/awesome-selfhosted/awesome-selfhosted
- https://start9.com
- https://radicle.xyz
