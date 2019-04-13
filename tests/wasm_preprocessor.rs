extern crate mdbook;
extern crate mdbook_preprocessor;
extern crate tempfile;
use mdbook_preprocessor::prelude::*;

use std::path::Path;
use tempfile::{Builder as TempFileBuilder, TempDir};

#[test]
fn test_wasm_preprocessor() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let book_dir = manifest_dir.join("book-example");
    let book = MDBook::load(&book_dir).expect("failed to load book-example");
    book.build_config.use_wasm_preprocessors = true;
    
}