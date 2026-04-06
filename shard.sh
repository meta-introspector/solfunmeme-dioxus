#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

SHARDS=71
DOCS=docs
SRC=target/dx/solfunmeme-dioxus/release/web/public

# Build release without base_path
sed -i 's/^base_path = "dioxus"/# base_path = "dioxus"/' Dioxus.toml
dx build --release --platform web
sed -i 's/^# base_path = "dioxus"/base_path = "dioxus"/' Dioxus.toml

# Clean and copy
rm -rf "$DOCS"
mkdir -p "$DOCS/assets/shards"

# Copy non-wasm assets
for f in "$SRC"/assets/*; do
    case "$f" in *.wasm|*/shards) ;; *) cp -r "$f" "$DOCS/assets/";; esac
done

# Find the WASM binary
WASM=$(find "$SRC/assets" -maxdepth 1 -name '*_bg*.wasm' | head -1)
[ -z "$WASM" ] && { echo "ERROR: no WASM found"; exit 1; }
echo "WASM: $WASM ($(wc -c < "$WASM") bytes)"

# Split into shards
TOTAL=$(wc -c < "$WASM")
split -b $(( (TOTAL + SHARDS - 1) / SHARDS )) -d -a 2 "$WASM" "$DOCS/assets/shards/shard_"

# Add .wasm extension
for f in "$DOCS"/assets/shards/shard_[0-9][0-9]; do
    mv "$f" "${f}.wasm"
done

# Generate manifest
python3 -c "
import hashlib,json,os
d='$DOCS/assets/shards'
s=[]
for i in range($SHARDS):
    p=os.path.join(d,f'shard_{i:02d}.wasm')
    data=open(p,'rb').read()
    s.append({'id':i,'size':len(data),'hash':hashlib.sha256(data).hexdigest()})
json.dump({'total_shards':$SHARDS,'total_bytes':sum(x['size'] for x in s),'shards':s},
          open(os.path.join(d,'manifest.json'),'w'))
print(f'✓ {$SHARDS} shards, {sum(x[\"size\"] for x in s)} bytes')
"

# Copy shard loader + generate index.html
cp assets/shard-loader.js "$DOCS/assets/"
JSFILE=$(basename "$DOCS"/assets/solfunmeme-dioxus-dxh*.js)
cat > "$DOCS/index.html" << HTML
<!DOCTYPE html>
<html><head>
<title>SOLFUNMEME DAO</title>
<meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<style>.dx-toast,#dx-toast-template{display:none!important}</style>
<script type="importmap">{"imports":{"env":"data:text/javascript,export default {}"}}</script>
</head><body>
<div id="main"><div style="display:flex;align-items:center;justify-content:center;height:100vh;flex-direction:column;font-family:sans-serif;background:#0a0a0a;color:#e0e0e0">
<div style="font-size:2.5rem;margin-bottom:1rem">☀️ SOLFUNMEME DAO</div>
<div style="width:280px;height:6px;background:#222;border-radius:3px;overflow:hidden"><div id="pbar" style="width:0%;height:100%;background:linear-gradient(90deg,#f7931a,#ffcc00);border-radius:3px;transition:width .15s"></div></div>
<div id="ptext" style="margin-top:.8rem;font-size:.85rem;opacity:.6">Loading 71 Gandalf shards…</div>
<div id="pdetail" style="margin-top:.3rem;font-size:.75rem;opacity:.4">0/71</div>
</div></div>
<script type="module">
import{loadShardedWasm}from"/assets/shard-loader.js";
const pb=document.getElementById("pbar"),pt=document.getElementById("ptext"),pd=document.getElementById("pdetail");
const buf=await loadShardedWasm((s,t,b,tb)=>{const p=Math.round(s/t*100);pb.style.width=p+"%";pt.textContent="Assembling… "+p+"%";pd.textContent=s+"/"+t+" · "+(b/1024|0)+"KB"});
pt.textContent="Initializing…";pb.style.width="100%";
const wr=new Response(buf,{headers:{"Content-Type":"application/wasm"}});
const of=window.fetch;window.fetch=function(u,o){return typeof u==="string"&&u.includes("_bg")&&u.endsWith(".wasm")?Promise.resolve(wr):of.call(this,u,o)};
await import("/assets/$JSFILE");
</script></body></html>
HTML
cp "$DOCS/index.html" "$DOCS/404.html"

echo "✓ docs/ ready: $(find "$DOCS" -type f | wc -l) files, $(du -sh "$DOCS" | cut -f1)"
