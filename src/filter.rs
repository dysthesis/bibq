//! TODO: Full filtering capabilities for bibtex entries

// TODO: Make a refinement type to enforce that years are correct
type Year = i32;

// TODO: Refinement type to ensure month is only from 1 to 12
type Month = i32;

#[derive(Debug)]
/// Reference: https://en.wikipedia.org/wiki/BibTeX#Entry_types
enum EntryType {
    /// An article from a journal or magazine.
    Article {
        author: String,
        title: String,
        journal: String,

        year: Year,

        // TODO: Check if i32 is too big or too small
        vol_or_number: Option<i32>,

        // TODO: Check if i32 is too big or too small
        pages: Option<i32>,

        note: Option<String>,

        // TODO: Likewise, check if there is a defined format for citekeys and whether one could
        // make a refinement type to check it at a type level
        key: Option<String>,
    },

    /// A book with an explicit publisher.
    Book {
        author_or_editor: String,
        title: String,
        publisher: String,
        year: Year,

        vol_or_number: Option<i32>,
        series: Option<String>,

        // TODO: Check if typesystem-level validation for addresses is possible
        address: Option<String>,

        // TODO: Is this the correct representation for edition
        edition: Option<i32>,

        month: Option<Month>,

        note: Option<String>,
        key: Option<String>,
    },

    /// A part of a book, usually untitled. May be a chapter (or session, etc.) and/or range of
    /// pages.
    InBook {
        author_or_editor: String,
        title: String,
        book_title: String,
        publisher: String,
        year: Year,

        editor: Option<String>,
        vol_or_number: Option<i32>,

        // TODO: Is this right?
        series: Option<String>,

        entry_type: Option<String>,
        address: Option<String>,
        edition: Option<i32>,
        month: Option<Month>,
        note: Option<String>,
        key: Option<String>,
    },
}
