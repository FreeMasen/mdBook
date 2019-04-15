#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
use mdbook_core::{book::Book, config::Config, errors::*, MDBOOK_VERSION};
use std::path::PathBuf;

pub use mdbook_core;
pub mod cmd;
pub mod wasm;

pub mod prelude {
    pub use crate::Preprocessor;
    pub use crate::PreprocessorContext;
    pub use crate::cmd::CmdPreprocessor;
    pub use mdbook_core::{
        book::*,
        errors::*,
        config::*,
    };

}
/// Extra information for a `Preprocessor` to give them more context when
/// processing a book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreprocessorContext {
    /// The location of the book directory on disk.
    pub root: PathBuf,
    /// The book configuration (`book.toml`).
    pub config: Config,
    /// The `Renderer` this preprocessor is being used with.
    pub renderer: String,
    /// The calling `mdbook` version.
    pub mdbook_version: String,
    #[serde(skip)]
    __non_exhaustive: (),
}

impl PreprocessorContext {
    /// Create a new `PreprocessorContext`.
    pub fn new(root: PathBuf, config: Config, renderer: String) -> Self {
        PreprocessorContext {
            root,
            config,
            renderer,
            mdbook_version: MDBOOK_VERSION.to_string(),
            __non_exhaustive: (),
        }
    }
}

/// An operation which is run immediately after loading a book into memory and
/// before it gets rendered.
pub trait Preprocessor {
    /// Get the `Preprocessor`'s name.
    fn name(&self) -> &str;

    /// Run this `Preprocessor`, allowing it to update the book before it is
    /// given to a renderer.
    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book>;

    /// A hint to `MDBook` whether this preprocessor is compatible with a
    /// particular renderer.
    ///
    /// By default, always returns `true`.
    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
