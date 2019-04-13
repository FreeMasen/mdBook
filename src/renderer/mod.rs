//! `mdbook`'s low level rendering interface.
//!
//! # Note
//!
//! You usually don't need to work with this module directly. If you want to
//! implement your own backend, then check out the [For Developers] section of
//! the user guide.
//!
//! The definition for [RenderContext] may be useful though.
//!
//! [For Developers]: https://rust-lang-nursery.github.io/mdBook/for_developers/index.html
//! [RenderContext]: struct.RenderContext.html

pub use self::html_handlebars::HtmlHandlebars;
mod html_handlebars;
