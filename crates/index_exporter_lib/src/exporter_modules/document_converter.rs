use std::error::Error;
use tantivy::schema::{TantivyDocument, Schema};
use tantivy::schema::document::CompactDocValue;
use crate::types::IndexedDocument;
use super::value_extractors::{extract_text_value, extract_u64_from_compact_doc_value};

pub fn document_to_indexed_document(doc: &TantivyDocument, schema: &Schema) -> Result<IndexedDocument, Box<dyn Error>> {
    let mut indexed_doc = IndexedDocument {
        path: None,
        content: None,
        emoji: None,
        line_start: None,
        line_end: None,
        chunk_type: None,
        language: None,
        content_hash: None,
        token_count: None,
        line_count: None,
        char_count: None,
        test_result: None,
    };
    
    for (field, value) in doc.field_values() {
        let field_name = schema.get_field_name(field);
        assign_field_value(&mut indexed_doc, field_name, &value);
    }
    
    Ok(indexed_doc)
}

fn assign_field_value(indexed_doc: &mut IndexedDocument, field_name: &str, value: &CompactDocValue) {
    match field_name {
        "path" => {
            indexed_doc.path = extract_text_value(value);
        },
        "content" => {
            indexed_doc.content = extract_text_value(value);
        },
        "emoji" => {
            indexed_doc.emoji = extract_text_value(value);
        },
        "line_start" => {
            indexed_doc.line_start = extract_u64_from_compact_doc_value(value);
        },
        "line_end" => {
            indexed_doc.line_end = extract_u64_from_compact_doc_value(value);
        },
        "chunk_type" => {
            indexed_doc.chunk_type = extract_text_value(value);
        },
        "language" => {
            indexed_doc.language = extract_text_value(value);
        },
        "content_hash" => {
            indexed_doc.content_hash = extract_text_value(value);
        },
        "token_count" => {
            indexed_doc.token_count = extract_u64_from_compact_doc_value(value);
        },
        "line_count" => {
            indexed_doc.line_count = extract_u64_from_compact_doc_value(value);
        },
        "char_count" => {
            indexed_doc.char_count = extract_u64_from_compact_doc_value(value);
        },
        "test_result" => {
            indexed_doc.test_result = extract_text_value(value);
        },
        _ => {}
    }
}
