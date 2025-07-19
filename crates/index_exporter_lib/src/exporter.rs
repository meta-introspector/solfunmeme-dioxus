use std::error::Error;
use std::path::PathBuf;
use tantivy::{Index, IndexReader, DocAddress};
use tantivy::directory::MmapDirectory;
use tantivy::schema::TantivyDocument;



use crate::types::{IndexedDocument, IndexStats, FieldStats};
use crate::exporter_modules::document_converter;

pub struct IndexExporter {
    index: Index,
    reader: IndexReader,
}

impl IndexExporter {
    pub fn new(index_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let index = Index::open(MmapDirectory::open(index_path)?)?;
        let reader = index.reader()?;
        
        Ok(Self { index, reader })
    }
    
    pub fn export_all_documents(&self, limit: usize) -> Result<Vec<IndexedDocument>, Box<dyn Error>> {
        let searcher = self.reader.searcher();
        let schema = self.index.schema();
        
        let total_docs = searcher.num_docs();
        let export_limit = if limit == 0 { total_docs as usize } else { limit.min(total_docs as usize) };
        
        println!("Exporting {} documents from index (total: {})", export_limit, total_docs);
        
        let mut documents = Vec::new();
        
        for doc_id in 0..export_limit {
            if let Ok(doc) = searcher.doc(DocAddress::new(0, doc_id as u32)) {
                if let Ok(indexed_doc) = self.document_to_indexed_document(&doc, &schema) {
                    documents.push(indexed_doc);
                }
            }
        }
        
        Ok(documents)
    }
    
    fn document_to_indexed_document(&self, doc: &TantivyDocument, schema: &tantivy::schema::Schema) -> Result<IndexedDocument, Box<dyn Error>> {
        document_converter::document_to_indexed_document(doc, schema)
    }
    
    pub fn get_stats(&self) -> Result<IndexStats, Box<dyn Error>> {
        let searcher = self.reader.searcher();
        let schema = self.index.schema();
        let total_docs = searcher.num_docs();
        
        let mut emoji_distribution = std::collections::HashMap::new();
        let mut language_distribution = std::collections::HashMap::new();
        let mut chunk_type_distribution = std::collections::HashMap::new();
        let mut path_distribution = std::collections::HashMap::new();
        
        // Collect statistics from all documents
        for doc_id in 0..total_docs {
            if let Ok(doc) = searcher.doc(DocAddress::new(0, doc_id as u32)) {
                if let Ok(indexed_doc) = self.document_to_indexed_document(&doc, &schema) {
                    // Count emojis
                    if let Some(emoji) = indexed_doc.emoji {
                        *emoji_distribution.entry(emoji).or_insert(0) += 1;
                    }
                    
                    // Count languages
                    if let Some(language) = indexed_doc.language {
                        *language_distribution.entry(language).or_insert(0) += 1;
                    }
                    
                    // Count chunk types
                    if let Some(chunk_type) = indexed_doc.chunk_type {
                        *chunk_type_distribution.entry(chunk_type).or_insert(0) += 1;
                    }
                    
                    // Count paths
                    if let Some(path) = indexed_doc.path {
                        *path_distribution.entry(path).or_insert(0) += 1;
                    }
                }
            }
        }
        
        // Create field statistics
        let fields = vec![
            FieldStats {
                field_name: "emoji".to_string(),
                non_null_count: emoji_distribution.values().sum(),
                unique_values: emoji_distribution.len(),
            },
            FieldStats {
                field_name: "language".to_string(),
                non_null_count: language_distribution.values().sum(),
                unique_values: language_distribution.len(),
            },
            FieldStats {
                field_name: "chunk_type".to_string(),
                non_null_count: chunk_type_distribution.values().sum(),
                unique_values: chunk_type_distribution.len(),
            },
            FieldStats {
                field_name: "path".to_string(),
                non_null_count: path_distribution.values().sum(),
                unique_values: path_distribution.len(),
            },
        ];
        
        Ok(IndexStats {
            total_documents: total_docs as usize,
            fields,
            emoji_distribution,
            language_distribution,
            chunk_type_distribution,
            path_distribution,
        })
    }
}