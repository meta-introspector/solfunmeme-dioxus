//use solfunmeme_extractor::components::extractor::types::CodeSnippet;
use solfunmeme_extractor::types::CodeSnippet;
use wasm_bindgen::JsValue;
use zip::write::{ZipWriter, FileOptions};
use js_sys::Array;
use web_sys::{BlobPropertyBag, Blob, Url};
use std::io::Write;

fn create_zip(snippets: &[CodeSnippet]) -> Result<String, JsValue> {
    let mut buffer = Vec::new();
    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buffer));
        let options = FileOptions::default();
            for (i, snippet) in snippets.iter().enumerate() {
                let file_name = format!("snippet_{}_{}.txt", i, snippet.language);
                zip.start_file(file_name, options).map_err(|e| JsValue::from(format!("{:?}", e)))?;
                zip.write_all(snippet.content.as_bytes()).map_err(|e| JsValue::from(format!("{:?}", e)))?;
            }
            zip.finish().map_err(|e| JsValue::from(format!("{:?}", e)))?;
        }
        let blob_parts = Array::new();
        blob_parts.push(&JsValue::from(js_sys::Uint8Array::from(buffer.as_slice())));
        let blob_property_bag = BlobPropertyBag::new();
        blob_property_bag.set_type("application/zip");
        let blob = Blob::new_with_buffer_source_sequence_and_options(&blob_parts, &blob_property_bag).map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let url = Url::create_object_url_with_blob(&blob).map_err(|e| JsValue::from(format!("{:?}", e)))?;
        Ok(url)
}
