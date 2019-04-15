extern crate mdbook;
extern crate mdbook_preprocessor;
extern crate tempfile;
extern crate mdbook_core;
extern crate env_logger;

use std::{
    process::Command,
    path::{Path, PathBuf}
};
use tempfile::Builder as TempFileBuilder;
use mdbook::MDBook;
use mdbook_core::book::BookItem;
use mdbook_preprocessor::{
    PreprocessorContext, 
    Preprocessor, 
    wasm::WasmPreprocessor
};

#[test]
fn test_wasm_preprocessor() {
    let _ = env_logger::try_init();
    let output = Command::new("cargo")
                .arg("build")
                .arg("-p")
                .arg("mdbook-wasm-preprocess-example")
                .arg("--target")
                .arg("wasm32-unknown-unknown")
                .output()
                .expect("Failed to build wasm");
    if !output.status.success() {
        panic!("Failed to build wasm {:?}", String::from_utf8_lossy(&output.stderr));
    }
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let wasm_path = manifest_dir
                        .join("target")
                        .join("wasm32-unknown-unknown")
                        .join("debug")
                        .join("mdbook_wasm_preprocess_example.wasm");
    let book_dir = manifest_dir.join("book-example");
    let plugins_dir = book_dir.join("preprocessors");
    if !plugins_dir.exists() {
        ::std::fs::create_dir(&plugins_dir).expect("Failed to create plugins dir");
    }
    let plugin_path = plugins_dir.join("mdbook_wasm_preprocess_example.wasm");
    ::std::fs::copy(wasm_path, plugin_path).expect("failed to copy wasm");

    let mut config: mdbook_core::config::Config = ::std::default::Default::default();
    config.build.use_wasm_preprocessors = true;
    let build_dir = TempFileBuilder::new().prefix("book").tempdir().unwrap();
    config.build.build_dir = PathBuf::from(build_dir.path());
    let md_book = MDBook::load_with_config(&book_dir, config).expect("failed to load book-example");
    let book = md_book.book.clone();
    let processor = WasmPreprocessor;
    let ctx = PreprocessorContext::new(
            md_book.root.clone(),
            md_book.config.clone(),
            "html".to_string(),
        );
    let processed = processor
        .run(&ctx, book)
        .expect("Error running preprocessor");
    for item in processed.iter() {
        if let BookItem::Chapter(ref p_ch) = item {
            if !p_ch.content.chars().any(|c| !c.is_alphabetic() || c.is_lowercase()) {
                panic!("chapter {} is not all uppercase\n{}", p_ch.name, p_ch.content);
            }
        }
    }
}