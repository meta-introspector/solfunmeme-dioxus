#!/usr/bin/env python3
"""
Vendor Dependency Migration Script

This script migrates the current flat vendor structure to a hierarchical
7-5-3-2 structure based on OSI-like layering principles.

Usage:
    python migrate_to_hierarchy.py [--dry-run] [--create-structure]
"""

import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import argparse

# Define the hierarchical structure
HIERARCHY = {
    "crypto": {
        "primitives": ["block-ciphers", "stream-ciphers", "AEADs", "MACs"],
        "protocols": ["rustls", "rust-native-tls", "hyper-tls", "hyper-rustls"],
        "keys": ["ed25519-dalek", "curve25519-dalek", "libsecp256k1", "borsh"],
        "hashes": ["BLAKE3", "hashes", "password-hashes", "adler2"],
        "utils": ["subtle", "constant_time_eq", "crypto-bigint", "cryptocorrosion"]
    },
    "system": {
        "os": ["linux-raw-sys", "rustix", "system-configuration-rs", "android_system_properties"],
        "io": ["mio", "polling", "parking", "event-listener"],
        "memory": ["memmap2-rs", "slab", "sharded-slab", "bumpalo"],
        "process": ["parking_lot", "concurrent-queue", "crossbeam", "rayon"],
        "utils": ["dunce", "path-clean", "tempfile", "same-file"]
    },
    "network": {
        "http": ["hyper", "axum", "tower", "tower-http", "reqwest"],
        "tls": ["rustls", "ring", "native-tls", "schannel-rs"],
        "websocket": ["tungstenite-rs", "reqwasm"],
        "protocols": ["h2", "http-body", "httparse", "httpdate"],
        "utils": ["reqwest", "tower", "hyper-util", "http"]
    },
    "data": {
        "serialization": ["serde", "bincode", "ciborium", "borsh-rs"],
        "storage": ["tantivy", "quickwit", "lancedb", "qdrant"],
        "structures": ["indexmap", "dashmap", "slotmap", "smallvec"],
        "formats": ["json", "toml", "csv", "xml", "yaml"],
        "utils": ["bytes", "byteorder", "bytemuck", "arrayvec"]
    },
    "compute": {
        "math": ["nalgebra", "ndarray", "linfa", "simba", "alga"],
        "algorithms": ["rayon", "itertools", "nom", "memchr"],
        "parallel": ["crossbeam", "parking_lot", "atomic-waker"],
        "optimization": ["fastrand", "ahash", "fxhash", "precomputed-hash"],
        "utils": ["num", "approx", "generic-array", "arrayref"]
    },
    "ui": {
        "framework": ["dioxus", "leptos", "manganis"],
        "components": ["dioxus-charts", "dioxus-motion", "dioxus-clipboard"],
        "rendering": ["stylo", "cssparser", "termcolor"],
        "interaction": ["keyboard-types", "cursor-icon", "copypasta"],
        "utils": ["clipboard-win", "smithay-clipboard", "client-toolkit"]
    },
    "ai": {
        "models": ["candle", "rustbert", "rust-bert", "fastembed-rs"],
        "nlp": ["nlprule", "vaporetto", "layered-nlp", "vtext"],
        "embedding": ["fastembed-rs", "rust-sbert", "rust-sentence-transformers"],
        "search": ["bm25", "keyword-extraction-rs", "model2vec-rs"],
        "utils": ["llms-from-scratch-rs", "rig-rust-llm-agents", "sophia_rs"]
    }
}

# Additional mappings for dependencies not in main categories
ADDITIONAL_MAPPINGS = {
    "crypto": {
        "primitives": ["block-modes", "sponges", "signatures"],
        "protocols": ["tls", "openssl-probe"],
        "keys": ["rust-bip39", "derivation-path", "ed25519-dalek-bip32"],
        "hashes": ["rust-crc32fast", "rust-siphash", "rust-fnv"],
        "utils": ["getrandom", "rand", "rngs"]
    },
    "system": {
        "os": ["android-tzdata", "iana-time-zone", "hermit-rs", "libhermit-rs"],
        "io": ["async-channel", "async-lock", "async-trait"],
        "memory": ["memoffset", "stable_deref_trait"],
        "process": ["atomic-waker", "event-listener-strategy"],
        "utils": ["atty", "autocfg", "cfg_aliases", "cfg_eval.rs"]
    },
    "network": {
        "http": ["reqwest", "hyper-util", "http-body"],
        "tls": ["ring-compat", "pki-types"],
        "websocket": [],
        "protocols": ["ipnet", "iri-string", "idna_adapter"],
        "utils": ["tower", "tower-http", "hyper-util"]
    },
    "data": {
        "serialization": ["serde-big-array", "serde-repr", "serde-wasm-bindgen"],
        "storage": ["memmap2-rs", "object", "addr2line"],
        "structures": ["enumset", "equivalent", "foreign-types"],
        "formats": ["quick-xml", "pulldown-cmark", "markdown-rs"],
        "utils": ["string-cache", "tendril", "futf", "encoding_rs"]
    },
    "compute": {
        "math": ["matrixmultiply", "algebra", "group", "ff"],
        "algorithms": ["longest-increasing-subsequence", "order-stat"],
        "parallel": ["portable-atomic", "pin-project", "pin-project-lite"],
        "optimization": ["crunchy", "rawpointer", "memoffset"],
        "utils": ["num-bigint", "num-complex", "num-rational", "num-integer"]
    },
    "ui": {
        "framework": ["gloo", "wasm-logger", "console_log"],
        "components": ["dioxus-motion", "dioxus-clipboard"],
        "rendering": ["stylo", "cssparser", "termcolor"],
        "interaction": ["keyboard-types", "cursor-icon"],
        "utils": ["clipboard-win", "smithay-clipboard"]
    },
    "ai": {
        "models": ["rust-bert", "fastembed-rs"],
        "nlp": ["eliza-rs", "gline-rs", "extractous"],
        "embedding": ["rust-sbert", "rust-sentence-transformers"],
        "search": ["bm25", "keyword-extraction-rs"],
        "utils": ["llms-from-scratch-rs", "rig-rust-llm-agents"]
    }
}

# Special cases and direct mappings
SPECIAL_MAPPINGS = {
    "solana-sdk": "crypto/protocols/stable",
    "solana-airdrop-service": "crypto/protocols/stable", 
    "agave": "crypto/protocols/stable",
    "agave-solana-validator": "crypto/protocols/stable",
    "steel": "compute/algorithms/stable",
    "rhai": "compute/algorithms/stable",
    "json-ld": "data/formats/stable",
    "tongrams-rs": "ai/nlp/stable",
    "vibrato": "ai/nlp/stable",
    "lean4": "compute/algorithms/stable",
    "rust": "system/os/stable",
    "syn-serde-rust": "data/serialization/stable",
    "fastembed-rs": "ai/embedding/stable",
    "rust-bert": "ai/models/stable",
    "syscall": "system/os/stable",
    "wasm-logger": "ui/framework/stable",
    "dunce": "system/utils/stable",
    "group": "compute/math/stable",
    "ff": "compute/math/stable",
    "zip": "data/formats/stable",
    "displaydoc": "ui/utils/stable",
    "dashmap": "data/structures/stable",
    "markdown-rs": "data/formats/stable",
    "winnow": "compute/algorithms/stable",
    "dioxus-motion": "ui/components/stable",
    "signal-hook": "system/process/stable",
    "uuid": "data/structures/stable",
    "dtoa-short": "data/utils/stable",
    "unicode-xid": "data/utils/stable",
    "unicode-width": "data/utils/stable",
    "unicode-segmentation": "data/utils/stable",
    "unicode-normalization": "data/utils/stable",
    "icu4x": "data/utils/stable",
    "feature-probe-rs": "system/utils/stable",
    "bv-rs": "data/structures/stable",
    "tracing": "system/utils/stable",
    "tokio": "system/process/stable",
    "slab": "system/memory/stable",
    "ahash": "compute/optimization/stable",
    "send_wrapper": "system/process/stable",
    "syn-serde": "data/serialization/stable",
    "portable-atomic": "system/process/stable",
    "pin-project-lite": "system/process/stable",
    "pin-project": "system/process/stable",
    "quick-xml": "data/formats/stable",
    "stable_deref_trait": "system/memory/stable",
    "tracing-wasm": "ui/framework/stable",
    "schannel-rs": "network/tls/stable",
    "rust-crc32fast": "crypto/hashes/stable",
    "sprs": "compute/math/stable",
    "system": "system/os/stable",
    "stake": "crypto/protocols/stable",
    "tungstenite-rs": "network/websocket/stable",
    "polling": "system/io/stable",
    "parking": "system/process/stable",
    "fastrand": "compute/optimization/stable",
    "event-listener-strategy": "system/io/stable",
    "event-listener": "system/io/stable",
    "concurrent-queue": "system/process/stable",
    "atomic-waker": "system/process/stable",
    "async-lock": "system/io/stable",
    "async-channel": "system/io/stable"
}

def create_directory_structure(base_path: Path, dry_run: bool = False) -> None:
    """Create the hierarchical directory structure."""
    print("Creating directory structure...")
    
    for layer, subcategories in HIERARCHY.items():
        for subcategory in subcategories:
            for component in ["core", "bindings", "extensions"]:
                for level in ["stable", "experimental"]:
                    path = base_path / layer / subcategory / component / level
                    if not dry_run:
                        path.mkdir(parents=True, exist_ok=True)
                    print(f"  {'[DRY RUN] ' if dry_run else ''}Created: {path}")

def find_dependency_location(dep_name: str) -> Optional[str]:
    """Find the appropriate location for a dependency in the hierarchy."""
    
    # Check special mappings first
    if dep_name in SPECIAL_MAPPINGS:
        return SPECIAL_MAPPINGS[dep_name]
    
    # Check main hierarchy
    for layer, subcategories in HIERARCHY.items():
        for subcategory, deps in subcategories.items():
            if dep_name in deps:
                return f"{layer}/{subcategory}/core/stable"
    
    # Check additional mappings
    for layer, subcategories in ADDITIONAL_MAPPINGS.items():
        for subcategory, deps in subcategories.items():
            if dep_name in deps:
                return f"{layer}/{subcategory}/core/stable"
    
    # Try to infer from name patterns
    if any(crypto_word in dep_name.lower() for crypto_word in ["crypto", "hash", "sha", "blake", "ed25519", "curve25519"]):
        return "crypto/primitives/core/stable"
    elif any(network_word in dep_name.lower() for network_word in ["http", "tls", "hyper", "reqwest", "tower"]):
        return "network/http/core/stable"
    elif any(system_word in dep_name.lower() for system_word in ["sys", "os", "linux", "android", "mio", "polling"]):
        return "system/os/core/stable"
    elif any(data_word in dep_name.lower() for data_word in ["serde", "json", "toml", "csv", "xml", "bincode"]):
        return "data/serialization/core/stable"
    elif any(compute_word in dep_name.lower() for compute_word in ["math", "algebra", "ndarray", "rayon", "crossbeam"]):
        return "compute/math/core/stable"
    elif any(ui_word in dep_name.lower() for ui_word in ["dioxus", "leptos", "ui", "css", "stylo"]):
        return "ui/framework/core/stable"
    elif any(ai_word in dep_name.lower() for ai_word in ["bert", "embed", "nlp", "ai", "ml", "model"]):
        return "ai/models/core/stable"
    
    return None

def migrate_dependencies(vendor_path: Path, dry_run: bool = False) -> Dict[str, List[str]]:
    """Migrate dependencies to the new hierarchical structure."""
    print("Migrating dependencies...")
    
    migration_log = {
        "migrated": [],
        "unmapped": [],
        "errors": []
    }
    
    # Get all subdirectories in vendor (excluding .git and README files)
    for item in vendor_path.iterdir():
        if not item.is_dir() or item.name.startswith('.'):
            continue
            
        dep_name = item.name
        new_location = find_dependency_location(dep_name)
        
        if new_location:
            new_path = vendor_path / new_location / dep_name
            old_path = vendor_path / dep_name
            
            try:
                if not dry_run:
                    # Create parent directories if they don't exist
                    new_path.parent.mkdir(parents=True, exist_ok=True)
                    # Move the directory
                    shutil.move(str(old_path), str(new_path))
                
                migration_log["migrated"].append(f"{dep_name} -> {new_location}")
                print(f"  {'[DRY RUN] ' if dry_run else ''}Migrated: {dep_name} -> {new_location}")
                
            except Exception as e:
                error_msg = f"Error migrating {dep_name}: {e}"
                migration_log["errors"].append(error_msg)
                print(f"  ERROR: {error_msg}")
        else:
            migration_log["unmapped"].append(dep_name)
            print(f"  UNMAPPED: {dep_name} (no location found)")
    
    return migration_log

def update_gitmodules(vendor_path: Path, dry_run: bool = False) -> None:
    """Update .gitmodules file with new paths."""
    print("Updating .gitmodules...")
    
    gitmodules_path = vendor_path.parent / ".gitmodules"
    if not gitmodules_path.exists():
        print("  No .gitmodules file found")
        return
    
    # Read current .gitmodules
    with open(gitmodules_path, 'r') as f:
        content = f.read()
    
    # Update paths based on new structure
    lines = content.split('\n')
    updated_lines = []
    
    for line in lines:
        if line.startswith('path = vendor/'):
            # Extract the old dependency name
            old_path = line.split(' = ')[1].strip()
            dep_name = old_path.split('/')[-1]
            
            # Find new location
            new_location = find_dependency_location(dep_name)
            if new_location:
                new_path = f"vendor/{new_location}/{dep_name}"
                updated_lines.append(f"path = {new_path}")
                print(f"  {'[DRY RUN] ' if dry_run else ''}Updated path: {old_path} -> {new_path}")
            else:
                updated_lines.append(line)
                print(f"  UNMAPPED in .gitmodules: {dep_name}")
        else:
            updated_lines.append(line)
    
    # Write updated content
    if not dry_run:
        with open(gitmodules_path, 'w') as f:
            f.write('\n'.join(updated_lines))
    
    print(f"  {'[DRY RUN] ' if dry_run else ''}Updated .gitmodules file")

def main():
    parser = argparse.ArgumentParser(description="Migrate vendor dependencies to hierarchical structure")
    parser.add_argument("--dry-run", action="store_true", help="Show what would be done without making changes")
    parser.add_argument("--create-structure", action="store_true", help="Only create the directory structure")
    parser.add_argument("--vendor-path", default="vendor", help="Path to vendor directory")
    
    args = parser.parse_args()
    
    vendor_path = Path(args.vendor_path)
    if not vendor_path.exists():
        print(f"Error: Vendor directory {vendor_path} does not exist")
        sys.exit(1)
    
    print(f"Vendor Migration Tool")
    print(f"====================")
    print(f"Vendor path: {vendor_path}")
    print(f"Dry run: {args.dry_run}")
    print()
    
    if args.create_structure:
        create_directory_structure(vendor_path, args.dry_run)
        return
    
    # Create directory structure
    create_directory_structure(vendor_path, args.dry_run)
    print()
    
    # Migrate dependencies
    migration_log = migrate_dependencies(vendor_path, args.dry_run)
    print()
    
    # Update .gitmodules
    update_gitmodules(vendor_path, args.dry_run)
    print()
    
    # Print summary
    print("Migration Summary")
    print("================")
    print(f"Migrated: {len(migration_log['migrated'])} dependencies")
    print(f"Unmapped: {len(migration_log['unmapped'])} dependencies")
    print(f"Errors: {len(migration_log['errors'])} errors")
    
    if migration_log['unmapped']:
        print("\nUnmapped dependencies:")
        for dep in migration_log['unmapped']:
            print(f"  - {dep}")
    
    if migration_log['errors']:
        print("\nErrors:")
        for error in migration_log['errors']:
            print(f"  - {error}")

if __name__ == "__main__":
    main() 