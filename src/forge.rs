#[cfg(not(target_arch = "wasm32"))]
pub mod forge {
    use serde::{Deserialize, Serialize};

    /// Generic git forge backend. Forgejo, Gitea, GitLab all supported.
    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub enum ForgeKind { Forgejo, Gitea, GitLab, Gogs }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct ForgeInstance {
        pub name: String,
        pub url: String,
        pub kind: ForgeKind,
        pub token: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ForgeRepo {
        pub full_name: String,
        pub description: String,
        pub clone_url: String,
        pub updated_at: String,
    }

    impl ForgeInstance {
        /// List repos via Forgejo/Gitea API v1 or GitLab API v4
        pub async fn list_repos(&self) -> Result<Vec<ForgeRepo>, String> {
            let client = reqwest::Client::new();
            let (url, auth_header) = match self.kind {
                ForgeKind::Forgejo | ForgeKind::Gitea | ForgeKind::Gogs => (
                    format!("{}/api/v1/repos/search?limit=50", self.url),
                    self.token.as_ref().map(|t| format!("token {}", t)),
                ),
                ForgeKind::GitLab => (
                    format!("{}/api/v4/projects?per_page=50", self.url),
                    self.token.as_ref().map(|t| format!("Bearer {}", t)),
                ),
            };
            let mut req = client.get(&url);
            if let Some(auth) = auth_header {
                req = req.header("Authorization", auth);
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let body = resp.text().await.map_err(|e| e.to_string())?;

            match self.kind {
                ForgeKind::Forgejo | ForgeKind::Gitea | ForgeKind::Gogs => {
                    #[derive(Deserialize)]
                    struct GiteaSearchResult { data: Vec<GiteaRepo> }
                    #[derive(Deserialize)]
                    struct GiteaRepo { full_name: String, description: String, clone_url: String, updated_at: String }
                    let result: GiteaSearchResult = serde_json::from_str(&body).map_err(|e| e.to_string())?;
                    Ok(result.data.into_iter().map(|r| ForgeRepo {
                        full_name: r.full_name, description: r.description,
                        clone_url: r.clone_url, updated_at: r.updated_at,
                    }).collect())
                }
                ForgeKind::GitLab => {
                    #[derive(Deserialize)]
                    struct GitLabProject { path_with_namespace: String, description: Option<String>, http_url_to_repo: String, last_activity_at: String }
                    let projects: Vec<GitLabProject> = serde_json::from_str(&body).map_err(|e| e.to_string())?;
                    Ok(projects.into_iter().map(|p| ForgeRepo {
                        full_name: p.path_with_namespace,
                        description: p.description.unwrap_or_default(),
                        clone_url: p.http_url_to_repo,
                        updated_at: p.last_activity_at,
                    }).collect())
                }
            }
        }
    }
}
