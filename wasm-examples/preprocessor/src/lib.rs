use wasmer_plugin::*;
use mdbook_core::book::{Book, BookItem};
extern "C" {
    fn print_str(ptr: *const u8, len: usize);
}

#[wasmer_plugin]
pub fn preprocess(mut book: Book) -> Book {
    book.sections = book.sections.clone().into_iter().map(updated_item).collect();
    book
}

fn updated_item(item: BookItem) -> BookItem {
    if let BookItem::Chapter(mut ch) = item {
        ch.content = ch.content.to_uppercase();
        debug_print(format!("{}", ch.content));
        BookItem::Chapter(ch)
    } else {
        item
    }
}

fn debug_print(s: String) {
    unsafe {
        print_str(s.as_ptr(), s.len());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mdbook_core::book::Chapter;
    #[test]
    fn test_preprocess() {
        let book = Book::with_chapters(vec![
            Chapter::new("Chapter One", format!("{}\n", "asdf".repeat(10)).repeat(100), "", vec![]).into(),
            Chapter::new("Chapter Two", format!("{}\n", "asdf".repeat(10)).repeat(100), "", vec![]).into(),
            Chapter::new("Chapter Three", format!("{}\n", "asdf".repeat(10)).repeat(100), "", vec![]).into(),
            Chapter::new("Chapter Four", format!("{}\n", "asdf".repeat(10)).repeat(100), "", vec![]).into(),
            Chapter::new("Chapter Five", format!("{}\n", "asdf".repeat(10)).repeat(100), "", vec![]).into(),
            Chapter::new("Chapter Six", format!("{}\n", "asdf".repeat(10)).repeat(100), "", vec![]).into(),
        ]);
        let updated = preprocess(book);
        assert!(updated.iter().filter_map(check_chapter).count() < 1);
        println!("{:?}", updated);
    }

    fn check_chapter(item: &BookItem) -> Option<&BookItem> {
        match item {
            BookItem::Chapter(ch) => {
                if ch.content.chars().filter(|c| c.is_lowercase()).count() > 0 {
                    Some(item)
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}