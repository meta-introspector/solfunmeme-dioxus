use tantivy::schema::document::CompactDocValue;
use tantivy::schema::Value;

pub fn extract_text_value(value: &CompactDocValue) -> Option<String> {
    value.as_str().map(|s| s.to_string())
}

pub fn extract_u64_from_compact_doc_value(value: &CompactDocValue) -> Option<u64> {
    value.as_u64()
}