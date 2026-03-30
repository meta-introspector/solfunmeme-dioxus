use anyhow::Result;
use quickwit::client::QuickwitClient;
use quickwit::document::Document;
use quickwit::query::Query;

pub struct QuickwitPlugin {
    client: QuickwitClient,
}

impl QuickwitPlugin {
    pub fn new(url: &str) -> Result<Self> {
        let client = QuickwitClient::new(url)?;
        Ok(QuickwitPlugin { client })
    }

    pub async fn index_document(&self, document: Document) -> Result<()> {
        self.client.index_document(document).await?;
        Ok(())
    }

    pub async fn search(&self, query: Query) -> Result<Vec<Document>> {
        let results = self.client.search(query).await?;
        Ok(results)
    }
}
