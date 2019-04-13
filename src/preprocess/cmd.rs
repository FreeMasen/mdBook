#[cfg(test)]
mod tests {
    use std::path::Path;
    use MDBook;
    use mdbook_preprocessor::cmd::CmdPreprocessor;
    use mdbook_preprocessor::PreprocessorContext;

    fn book_example() -> MDBook {
        let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("book-example");
        MDBook::load(example).unwrap()
    }

    #[test]
    fn round_trip_write_and_parse_input() {
        let cmd = CmdPreprocessor::new("test".to_string(), "test".to_string());
        let md = book_example();
        let ctx = PreprocessorContext::new(
            md.root.clone(),
            md.config.clone(),
            "some-renderer".to_string(),
        );

        let mut buffer = Vec::new();
        cmd.write_input(&mut buffer, &md.book, &ctx).unwrap();

        let (got_ctx, got_book) = CmdPreprocessor::parse_input(buffer.as_slice()).unwrap();

        assert_eq!(got_book, md.book);
        assert_eq!(got_ctx, ctx);
    }
}
