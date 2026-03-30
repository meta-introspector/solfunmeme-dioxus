pub mod args;
pub mod file_finder;
pub mod processor;
pub mod output_handler;

pub use args::DocProcessorArgs;
pub use file_finder::find_doc_files;
pub use processor::process_documentation_files;
pub use output_handler::save_processed_document;
