const N=71,B="/assets/shards/shard_";
export async function loadShardedWasm(cb){
const m=await(await fetch("/assets/shards/manifest.json")).json();
let loaded=0;const bufs=new Array(N);
for(let b=0;b<N;b+=8){const p=[];
for(let i=b;i<Math.min(b+8,N);i++)p.push(fetch(B+String(i).padStart(2,"0")+".wasm").then(r=>r.arrayBuffer()).then(buf=>{bufs[i]=new Uint8Array(buf);loaded+=buf.byteLength;if(cb)cb(i+1,N,loaded,m.total_bytes)}));
await Promise.all(p)}
const c=new Uint8Array(m.total_bytes);let o=0;
for(let i=0;i<N;i++){c.set(bufs[i],o);o+=bufs[i].length}
return c.buffer}
