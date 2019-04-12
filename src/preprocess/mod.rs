//! Book preprocessing.

pub use self::cmd::CmdPreprocessor;
pub use self::index::IndexPreprocessor;
pub use self::links::LinkPreprocessor;

mod cmd;
mod index;
mod links;

use mdbook_core::Book;
use config::Config;
use errors::*;

use std::path::PathBuf;

