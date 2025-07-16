pub mod args;
pub mod extractor;
pub mod generator;

pub use args::DocTestGeneratorArgs;
pub use extractor::{CodeExample, extract_code_examples};
pub use generator::generate_doc_tests;
