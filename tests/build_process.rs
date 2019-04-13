extern crate mdbook;
extern crate mdbook_core;
extern crate mdbook_preprocessor;
extern crate mdbook_renderer;

mod dummy_book;

use dummy_book::DummyBook;
use mdbook_core::book::Book;
use mdbook_core::config::Config;
use mdbook_core::errors::*;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use mdbook_renderer::{RenderContext, Renderer};
use mdbook::MDBook;
use std::sync::{Arc, Mutex};

struct Spy(Arc<Mutex<Inner>>);

#[derive(Debug, Default)]
struct Inner {
    run_count: usize,
    rendered_with: Vec<String>,
}

impl Preprocessor for Spy {
    fn name(&self) -> &str {
        "dummy"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let mut inner = self.0.lock().unwrap();
        inner.run_count += 1;
        inner.rendered_with.push(ctx.renderer.clone());
        Ok(book)
    }
}

impl Renderer for Spy {
    fn name(&self) -> &str {
        "dummy"
    }

    fn render(&self, _ctx: &RenderContext) -> Result<()> {
        let mut inner = self.0.lock().unwrap();
        inner.run_count += 1;
        Ok(())
    }
}

#[test]
fn mdbook_runs_preprocessors() {
    let spy: Arc<Mutex<Inner>> = Default::default();

    let temp = DummyBook::new().build().unwrap();
    let cfg = Config::default();

    let mut book = MDBook::load_with_config(temp.path(), cfg).unwrap();
    book.with_preprecessor(Spy(Arc::clone(&spy)));
    book.build().unwrap();

    let inner = spy.lock().unwrap();
    assert_eq!(inner.run_count, 1);
    assert_eq!(inner.rendered_with.len(), 1);
    assert_eq!(
        "html", inner.rendered_with[0],
        "We should have been run with the default HTML renderer"
    );
}

#[test]
fn mdbook_runs_renderers() {
    let spy: Arc<Mutex<Inner>> = Default::default();

    let temp = DummyBook::new().build().unwrap();
    let cfg = Config::default();

    let mut book = MDBook::load_with_config(temp.path(), cfg).unwrap();
    book.with_renderer(Spy(Arc::clone(&spy)));
    book.build().unwrap();

    let inner = spy.lock().unwrap();
    assert_eq!(inner.run_count, 1);
}
