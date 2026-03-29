#[cfg(not(target_arch = "wasm32"))]
pub mod server {
    use axum::{Router, Json, extract::State, routing::{get, post}};
    use serde::{Deserialize, Serialize};
    use sha2::{Sha256, Digest};
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    // ── State ───────────────────────────────────────────────────

    #[derive(Clone)]
    pub struct NodeState {
        pub pastes: Arc<Mutex<HashMap<String, Paste>>>,
        pub peers: Arc<Mutex<Vec<String>>>,
        pub forges: Vec<crate::forge::forge::ForgeInstance>,
        pub port: u16,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Paste {
        pub id: String,
        pub content: String,
        pub timestamp: i64,
    }

    #[derive(Serialize)]
    pub struct NodeStatus {
        pub version: &'static str,
        pub peers: usize,
        pub pastes: usize,
        pub uptime_secs: u64,
    }

    #[derive(Serialize)]
    pub struct ZkperfWitness {
        pub timestamp: i64,
        pub commitment: String,
        pub latency_bucket: u8,
        pub orbifold: [u64; 3],
        pub crown_product: u64,
    }

    #[derive(Deserialize)]
    pub struct StegoEncodeReq {
        pub data: String,
        pub format: Option<String>,
    }

    #[derive(Serialize)]
    pub struct StegoEncodeResp {
        pub carrier: String,
        pub format: String,
        pub bytes: usize,
    }

    // ── Routes ──────────────────────────────────────────────────

    pub fn router(state: NodeState) -> Router {
        Router::new()
            .route("/", get(status))
            .route("/status", get(status))
            .route("/paste", post(paste_create))
            .route("/paste/{id}", get(paste_get))
            .route("/zkperf", get(zkperf_witness))
            .route("/stego/encode", post(stego_encode))
            .route("/stego/decode", post(stego_decode))
            .route("/peers", get(list_peers))
            .route("/ipfs/{cid}", get(ipfs_get))
            .route("/ipfs", post(ipfs_add))
            .route("/ipfs/publish", post(ipfs_publish))
            .route("/forge/repos", get(forge_repos))
            .with_state(state)
    }

    async fn status(State(s): State<NodeState>) -> Json<NodeStatus> {
        Json(NodeStatus {
            version: env!("CARGO_PKG_VERSION"),
            peers: s.peers.lock().unwrap().len(),
            pastes: s.pastes.lock().unwrap().len(),
            uptime_secs: 0, // TODO: track
        })
    }

    async fn paste_create(State(s): State<NodeState>, Json(body): Json<Paste>) -> Json<Paste> {
        let mut hasher = Sha256::new();
        hasher.update(body.content.as_bytes());
        let id = hex::encode(&hasher.finalize()[..8]);
        let paste = Paste {
            id: id.clone(),
            content: body.content,
            timestamp: chrono::Utc::now().timestamp(),
        };
        s.pastes.lock().unwrap().insert(id, paste.clone());
        Json(paste)
    }

    async fn paste_get(State(s): State<NodeState>, axum::extract::Path(id): axum::extract::Path<String>) -> Json<Option<Paste>> {
        Json(s.pastes.lock().unwrap().get(&id).cloned())
    }

    async fn zkperf_witness() -> Json<ZkperfWitness> {
        let now = chrono::Utc::now().timestamp();
        let mut hasher = Sha256::new();
        hasher.update(now.to_le_bytes());
        let commitment = hex::encode(hasher.finalize());
        Json(ZkperfWitness {
            timestamp: now,
            commitment,
            latency_bucket: 1,
            orbifold: [(now as u64) % 71, (now as u64) % 59, (now as u64) % 47],
            crown_product: 196_883,
        })
    }

    async fn stego_encode(Json(req): Json<StegoEncodeReq>) -> Json<StegoEncodeResp> {
        use erdfa_publish::stego::ZeroWidthText;
        use erdfa_publish::StegoPlugin;
        let plugin = ZeroWidthText;
        let carrier = plugin.encode(req.data.as_bytes());
        Json(StegoEncodeResp {
            carrier: String::from_utf8_lossy(&carrier).to_string(),
            format: "zwc-text".into(),
            bytes: carrier.len(),
        })
    }

    async fn stego_decode(body: String) -> Json<Option<String>> {
        use erdfa_publish::stego::ZeroWidthText;
        use erdfa_publish::StegoPlugin;
        let plugin = ZeroWidthText;
        let decoded = plugin.decode(body.as_bytes()).map(|b| String::from_utf8_lossy(&b).to_string());
        Json(decoded)
    }

    async fn list_peers(State(s): State<NodeState>) -> Json<Vec<String>> {
        Json(s.peers.lock().unwrap().clone())
    }

    async fn ipfs_get(axum::extract::Path(cid): axum::extract::Path<String>) -> Json<Option<String>> {
        use erdfa_publish::ipfs;
        let data = ipfs::ipfs_cat(&cid).map(|b| String::from_utf8_lossy(&b).to_string());
        Json(data)
    }

    async fn ipfs_add(body: String) -> Json<Option<String>> {
        use erdfa_publish::ipfs;
        Json(ipfs::ipfs_add(&body))
    }

    #[derive(Deserialize)]
    pub struct PublishReq {
        pub cid: String,
        pub acl: String, // "public", "holder", "private"
    }

    #[derive(Serialize)]
    pub struct PublishResp {
        pub cid: String,
        pub acl: String,
        pub signature: String,
        pub merkle_root: String,
        pub public_header: PublicHeader,
    }

    #[derive(Serialize)]
    pub struct PublicHeader {
        pub cid: String,
        pub acl: String,
        pub size: usize,
        pub signed_by: String,
        pub timestamp: i64,
    }

    /// Review + sign + publish an IPFS object.
    /// Only Public-tier objects get the full content exposed.
    /// All tiers get a signed public header with CID + ACL + merkle root.
    async fn ipfs_publish(Json(req): Json<PublishReq>) -> Json<Result<PublishResp, String>> {
        use erdfa_publish::ipfs;
        use erdfa_publish::privacy::{PrivacyShard, SignedPrivacyShard};

        let data = match ipfs::ipfs_cat(&req.cid) {
            Some(d) => d,
            None => return Json(Err("CID not found locally".into())),
        };

        let acl = match req.acl.as_str() {
            "public" => "Public",
            "holder" => "Holder",
            "private" => "Private",
            _ => return Json(Err("acl must be public|holder|private".into())),
        };

        // Build privacy shard with field-level control
        let pairs: Vec<(String, String)> = vec![
            ("cid".into(), req.cid.clone()),
            ("acl".into(), acl.into()),
            ("size".into(), data.len().to_string()),
            ("content_hash".into(), hex::encode(sha2::Sha256::digest(&data))),
        ];
        let shard = PrivacyShard::from_pairs("publish", &pairs, vec![]);

        // Sign with ML-DSA-44
        let signed = match SignedPrivacyShard::sign(shard) {
            Ok(s) => s,
            Err(e) => return Json(Err(format!("signing failed: {}", e))),
        };

        let header = PublicHeader {
            cid: req.cid.clone(),
            acl: acl.into(),
            size: data.len(),
            signed_by: hex::encode(&signed.public_key[..16]),
            timestamp: chrono::Utc::now().timestamp(),
        };

        Json(Ok(PublishResp {
            cid: req.cid,
            acl: acl.into(),
            signature: hex::encode(&signed.signature[..32]),
            merkle_root: signed.shard.merkle_root.clone(),
            public_header: header,
        }))
    }

    async fn forge_repos(State(s): State<NodeState>) -> Json<Vec<crate::forge::forge::ForgeRepo>> {
        let mut all = vec![];
        for instance in &s.forges {
            if let Ok(repos) = instance.list_repos().await {
                all.extend(repos);
            }
        }
        Json(all)
    }

    // ── Start ───────────────────────────────────────────────────

    pub async fn start(port: u16) -> String {
        let state = NodeState {
            pastes: Arc::new(Mutex::new(HashMap::new())),
            peers: Arc::new(Mutex::new(vec![])),
            forges: vec![
                crate::forge::forge::ForgeInstance {
                    name: "local-forgejo".into(),
                    url: "http://localhost:3000".into(),
                    kind: crate::forge::forge::ForgeKind::Forgejo,
                    token: std::env::var("FORGEJO_TOKEN").ok(),
                },
            ],
            port,
        };
        let app = router(state);
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        let local = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        local
    }
}
