//! This preprocessor will enable developers to 
//! use a wasm module to preprocess the contents
//! of an mdbook. The plugin developer would 
//! simply need to create a rust library with
//! the following function
//! ```rust
//! extern crate mdbook_preprocessor;
//! extern crate wasmer_plugin;
//! use mdbook_preprocessor::prelude::*;
//! use wasmer_plugin::*;
//!
//! pub fn preprocess(book: Book) -> Bool {
//!     //do your things
//!     book
//! } 
//! ```
use super::{
    Preprocessor, 
    PreprocessorContext
};
use mdbook_core::book::Book;
use mdbook_core::errors::*;
use std::{
    io::Read,
    fs::File,
    path::PathBuf
};
use bincode;

static NAME: &str = "Wasm Preprocessor";
const START: usize = 5;

pub struct WasmPreprocessor;

impl Preprocessor for WasmPreprocessor {
    fn name(&self) -> &str {
        NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        debug!("Running Wasm Preprocessors");
        // All wasm preprocessors will be stored in a file 
        // named preprocessors in the book's root
        let preprocessors_path = ctx.root.join("preprocessors");
        // If that doesn't exist, move along
        if !preprocessors_path.exists() {
            debug!("No preprocessors in book root, skipping");
            return Ok(book);
        }
        // Loop over the contents, running the sub_processor on each
        // of the .wasm files in preprocessors
        for entry in ::std::fs::read_dir(preprocessors_path)?.filter_map(|e| e.ok()) {
            if entry.file_type()?.is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "wasm" {
                        debug!("Attempting to execute {:?}", path.file_name());
                        book = Self::run_sub_processor(book, &path)?;
                    }
                }
            }
        }
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        match renderer {
            "html" => true,
            _ => false,
        }
    }
}

impl WasmPreprocessor {
    /// Run a wasm preprocessor
    /// This will read the wasm module from disc, instantiate it
    /// pass the book into the wasm context, execute the plugin's
    /// `preprocess` function and extract the updated value
    fn run_sub_processor(book: Book, path: &PathBuf) -> Result<Book> {
        let mut inst = Self::read_and_instantiate_wasm(path)?;
        let len = Self::serialize_and_inject_book(&mut inst, &book)?;
        let ret = Self::run_wasm(&mut inst, len)?;
        Ok(ret)
    }

    /// This handles reading the bytes from disc and
    /// instantiating the wasm module
    fn read_and_instantiate_wasm(path: &PathBuf) -> Result<wasmer_runtime::Instance> {
        debug!("read_and_instantiate_wasm");
        let mut wasm = Vec::new();
        let mut f = File::open(path)?;
        f.read_to_end(&mut wasm)?;
        let inst = wasmer_runtime::instantiate(&wasm, &wasmer_runtime::imports! {
            // Define the "env" namespace that was implicitly used
            // by our sample application.
            "env" => {
                // name        // the func! macro autodetects the signature
                "print_str" => wasmer_runtime::func!(print_str),
            },
        }).map_err(|e| Error::from(format!("failed to instantiate {:?}\n{}", path, e)))?;
        Ok(inst)
    }

    /// This will convert the book into bytes via the `bincode::serialize` function
    /// and inject those bytes into the instance's wasm memory
    fn serialize_and_inject_book(inst: &mut wasmer_runtime::Instance, book: &Book) -> Result<usize> {
        debug!("serialize_and_inject_book");
        let serialized = bincode::serialize(book).map_err(|e| Error::from(format!("error serializing {}", e)))?;
        let mem = inst.context_mut().memory(0);
        let len = serialized.len();
        debug!("serialized length: {}", len);
        // Notice that we are starting at byte 1 and not byte 0
        // This is because we are going to reserve byte 0 for
        // the new length of the return value 
        for (cell, byte) in mem.view()[START..len + START].iter().zip(serialized.iter()) {
            cell.set(*byte)
        }
        Ok(len)
    }

    /// This will execute the plugin's `preprocess` function and extract the
    /// updated book from the wasm instance's memory
    fn run_wasm(inst: &mut wasmer_runtime::Instance, len: usize) -> Result<Book> {
        debug!("run_wasm");
        let func = inst.func::<(i32, u32), i32>("_preprocess").map_err(|e| Error::from(format!("failed to bind _interact \n{}", e)))?;
        let ptr = func.call(START as i32, len as u32).map_err(|e| Error::from(format!("failed to execute _preprocess\n{}", e)))?;
        Self::extract_value(inst, ptr)
    }

    /// Here we will pull the bincode bytes out of the wasm instance's memory
    /// and serialize it back into an mdbook
    fn extract_value(inst: &wasmer_runtime::Instance, ptr: i32) -> Result<Book> {
        debug!("extract_value {}", ptr);
        let updated_mem = inst.context().memory(0);
        let view = updated_mem.view();
        // We need to be able to pull out the first byte of memory
        // because this is where the new length of the return value
        // will live.
        let mut new_len_bytes: [u8;4] = [0;4];
        for i in 0..4 {
            let v = view.get(i + 1).map(|c| c.get()).ok_or(Error::from(format!("unable to get new length part {}", i)))?;;
            new_len_bytes[i] = v;
        }
        trace!("new_len_bytes: {:?}", new_len_bytes);
        let new_len: u32 = u32::from_ne_bytes(new_len_bytes);
        debug!("new length: {}", new_len);
        let buf: Vec<u8> = view[ptr as usize..ptr as usize + new_len as usize].iter().map(|c| c.get()).collect();
        trace!("{:?}", buf);
        let updated_book = bincode::deserialize(&buf).map_err(|e| Error::from(format!("Unable to reconstruct book after\n{}", e)))?;
        Ok(updated_book)
    }
}

fn print_str(ctx: &mut wasmer_runtime::Ctx, ptr: u32, len: u32) {
    let memory = ctx.memory(0);
    let str_vec: Vec<_> = memory.view()[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(|cell| cell.get())
        .collect();
    let string = String::from_utf8_lossy(&str_vec).to_string();
    println!("wasm: {}", string);
}