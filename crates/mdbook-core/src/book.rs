use std::{
    collections::VecDeque,
    fmt,
    fmt::{Display, Formatter},
    iter::FromIterator,
    ops::{Deref, DerefMut},
    path::PathBuf,
};
/// A dumb tree structure representing a book.
///
/// For the moment a book is just a collection of `BookItems` which are
/// accessible by either iterating (immutably) over the book with [`iter()`], or
/// recursively applying a closure to each section to mutate the chapters, using
/// [`for_each_mut()`].
///
/// [`iter()`]: #method.iter
/// [`for_each_mut()`]: #method.for_each_mut
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Book {
    /// The sections in this book.
    pub sections: Vec<BookItem>,
    __non_exhaustive: (),
}

impl Book {
    /// Create an empty book.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_chapters(sections: Vec<BookItem>) -> Self {
        Self {
            sections,
            __non_exhaustive: (),
        }
    }

    /// Get a depth-first iterator over the items in the book.
    pub fn iter(&self) -> BookItems {
        BookItems {
            items: self.sections.iter().collect(),
        }
    }

    /// Recursively apply a closure to each item in the book, allowing you to
    /// mutate them.
    ///
    /// # Note
    ///
    /// Unlike the `iter()` method, this requires a closure instead of returning
    /// an iterator. This is because using iterators can possibly allow you
    /// to have iterator invalidation errors.
    pub fn for_each_mut<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut BookItem),
    {
        for_each_mut(&mut func, &mut self.sections);
    }

    /// Append a `BookItem` to the `Book`.
    pub fn push_item<I: Into<BookItem>>(&mut self, item: I) -> &mut Self {
        self.sections.push(item.into());
        self
    }
}

pub fn for_each_mut<'a, F, I>(func: &mut F, items: I)
where
    F: FnMut(&mut BookItem),
    I: IntoIterator<Item = &'a mut BookItem>,
{
    for item in items {
        if let &mut BookItem::Chapter(ref mut ch) = item {
            for_each_mut(func, &mut ch.sub_items);
        }

        func(item);
    }
}

/// Enum representing any type of item which can be added to a book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BookItem {
    /// A nested chapter.
    Chapter(Chapter),
    /// A section separator.
    Separator,
}

impl From<Chapter> for BookItem {
    fn from(other: Chapter) -> BookItem {
        BookItem::Chapter(other)
    }
}

/// The representation of a "chapter", usually mapping to a single file on
/// disk however it may contain multiple sub-chapters.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    /// The chapter's name.
    pub name: String,
    /// The chapter's contents.
    pub content: String,
    /// The chapter's section number, if it has one.
    pub number: Option<SectionNumber>,
    /// Nested items.
    pub sub_items: Vec<BookItem>,
    /// The chapter's location, relative to the `SUMMARY.md` file.
    pub path: PathBuf,
    /// An ordered list of the names of each chapter above this one, in the hierarchy.
    pub parent_names: Vec<String>,
}

impl Chapter {
    /// Create a new chapter with the provided content.
    pub fn new<P: Into<PathBuf>>(
        name: &str,
        content: String,
        path: P,
        parent_names: Vec<String>,
    ) -> Chapter {
        Chapter {
            name: name.to_string(),
            content,
            path: path.into(),
            parent_names,
            ..Default::default()
        }
    }
}

/// A depth-first iterator over the items in a book.
///
/// # Note
///
/// This struct shouldn't be created directly, instead prefer the
/// [`Book::iter()`] method.
///
/// [`Book::iter()`]: struct.Book.html#method.iter
pub struct BookItems<'a> {
    items: VecDeque<&'a BookItem>,
}

impl<'a> Iterator for BookItems<'a> {
    type Item = &'a BookItem;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.pop_front();

        if let Some(&BookItem::Chapter(ref ch)) = item {
            // if we wanted a breadth-first iterator we'd `extend()` here
            for sub_item in ch.sub_items.iter().rev() {
                self.items.push_front(sub_item);
            }
        }

        item
    }
}

impl Display for Chapter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(ref section_number) = self.number {
            write!(f, "{} ", section_number)?;
        }

        write!(f, "{}", self.name)
    }
}

/// A section number like "1.2.3", basically just a newtype'd `Vec<u32>` with
/// a pretty `Display` impl.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SectionNumber(pub Vec<u32>);

impl Display for SectionNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "0")
        } else {
            for item in &self.0 {
                write!(f, "{}.", item)?;
            }
            Ok(())
        }
    }
}

impl Deref for SectionNumber {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SectionNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<u32> for SectionNumber {
    fn from_iter<I: IntoIterator<Item = u32>>(it: I) -> Self {
        SectionNumber(it.into_iter().collect())
    }
}
