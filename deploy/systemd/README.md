# Solfunmeme systemd units

Installed by go-live-remediation.sh (default: dry-run, --apply to install).

## solfunmeme-prefetch.{service,timer}

Warms the Nix store with flake inputs so the first request after a cold boot
does not wait on the network.

- Runs as: DynamicUser=yes (no static user required)
- Cache: /var/cache/solfunmeme-prefetch (StateDirectory + CacheDirectory)
- Trigger: 2min after boot, then every 6h (Persistent=true catches missed runs)
- Command: nix flake archive from /opt/solfunmeme/flake.nix

### Adjust before Go-Live

Three knobs assume defaults - confirm with the repo owner:
1. What is prefetched - currently nix flake archive (flake inputs only).
   If we also want NAR/cache-path warmup, add nix store prefetch-file per URL.
2. Who runs - DynamicUser=yes is safe; if the repo requires a real
   solfunmeme user (e.g. for secrets), switch to User=solfunmeme + Group=solfunmeme.
3. Cadence - 6h. Change OnUnitActiveSec= to 12h/24h if the flake is stable.

### Portability

ExecStart uses `/bin/sh -c "exec nix ..."` so systemd does not
validate the nix binary path at unit-load time. Works on Ubuntu+Determinate
(nix at /nix/var/nix/profiles/default/bin/nix) and NixOS
(nix at /run/current-system/sw/bin/nix) without changes.
