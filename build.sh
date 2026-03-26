#!/usr/bin/env bash
set -euo pipefail
cd /mnt/data1/meta-introspector/submodules/solfunmeme-dioxus
exec nix develop --command bash shard.sh
