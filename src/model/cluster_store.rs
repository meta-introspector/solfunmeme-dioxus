use crate::model::AdapterCluster;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct ClusterStore {
    clusters: Vec<AdapterCluster>,
    active_cluster: AdapterCluster,
}

impl ClusterStore {
    pub fn new(clusters: Vec<AdapterCluster>) -> Self {
        Self {
            clusters,
            active_cluster: AdapterCluster::default(),
        }
    }

    pub fn get_clusters(&self) -> &[AdapterCluster] {
        self.clusters.as_slice()
    }

    pub fn add_cluster(&mut self, cluster: AdapterCluster) -> Result<&mut Self, String> {
        let cluster_exists = self.clusters.iter().any(|inner_cluster| {
            inner_cluster.name().as_bytes() == cluster.name().as_bytes()
                || inner_cluster.endpoint().as_bytes() == cluster.endpoint().as_bytes()
        });

        if cluster_exists {
            Err(String::from(
                "Cluster exists, make sure endpoint or name are not the same",
            ))
        } else {
            self.clusters.push(cluster);
            Ok(self)
        }
    }

    pub fn set_active_cluster(&mut self, cluster: AdapterCluster) -> &mut Self {
        self.active_cluster = cluster;
        self
    }

    pub fn active_cluster(&self) -> &AdapterCluster {
        &self.active_cluster
    }

    pub fn add_clusters(&mut self, clusters: Vec<AdapterCluster>) -> Result<(), String> {
        clusters.into_iter().try_for_each(|cluster| {
            self.add_cluster(cluster)?;
            Ok::<(), String>(())
        })
    }

    pub fn get_cluster(&self, name: &str) -> Option<&AdapterCluster> {
        self.clusters.iter().find(|cluster| cluster.name() == name)
    }

    pub fn remove_cluster(&mut self, cluster_name: &str) -> Option<AdapterCluster> {
        self.clusters
            .iter()
            .position(|current_cluster| current_cluster.name().as_bytes() == cluster_name.as_bytes())
            .map(|index| self.clusters.remove(index))
    }
}
