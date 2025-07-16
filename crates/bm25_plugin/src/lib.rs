use anyhow::Result;
use bm25::BM25;

pub struct Bm25Plugin {
    bm25: BM25,
}

impl Bm25Plugin {
    pub fn new(corpus: Vec<Vec<String>>) -> Result<Self> {
        let bm25 = BM25::new(corpus);
        Ok(Bm25Plugin { bm25 })
    }

    pub fn calculate_score(&self, query: &[String], doc_idx: usize) -> f64 {
        self.bm25.score(query, doc_idx)
    }

    pub fn search(&self, query: &[String]) -> Vec<(usize, f64)> {
        self.bm25.search(query)
    }
}
