use std::fmt::Write;

use textwrap::{fill, Options};

use crate::args::{Book as ArgBook, Chapter, Verses};
use crate::{Error, BOOKS};

impl TryFrom<ArgBook> for String {
    type Error = Error;

    fn try_from(arg_book: ArgBook) -> Result<Self, Self::Error> {
        let (alias, num) = arg_book.alias_num();
        let book = BOOKS[num];

        let mut out = String::new();
        let indent = " ".repeat(alias.len() + 9);
        let opts = Options::new(80).subsequent_indent(&indent);

        if let Some(ch) = arg_book.chapter() {
            match ch {
                Chapter::Single {
                    chapter,
                    verses: Some(verses),
                } => {
                    // print verses of a chapter
                    let check_verse = |num: usize| {
                        let verses = book[*chapter].len();
                        if num > verses {
                            Err(Error::InvalidVerse {
                                num,
                                title: book.title.to_string(),
                                chapter: *chapter,
                                verses,
                            })
                        } else {
                            Ok(())
                        }
                    };
                    match verses {
                        Verses::Single(verse) => {
                            check_verse(*verse)?;
                            write_verse(
                                &mut out,
                                &opts,
                                alias,
                                *chapter,
                                *verse,
                                book[*chapter][*verse],
                            )?;
                        }
                        Verses::Range(range) => {
                            for v in book[*chapter][range.clone()].iter().enumerate() {
                                write_verse(
                                    &mut out,
                                    &opts,
                                    alias,
                                    *chapter,
                                    v.0 + range.start(),
                                    v.1,
                                )?;
                            }
                        }
                    }
                }
                Chapter::Single { chapter, .. } => {
                    // print a single chapter
                    for v in book[*chapter].iter().enumerate() {
                        write_verse(&mut out, &opts, alias, *chapter, v.0, v.1)?;
                    }
                }
                Chapter::Range(range) => {
                    // print a range of chapters
                    for ch in book[range.clone()].iter().enumerate() {
                        for v in ch.1.iter().enumerate() {
                            write_verse(&mut out, &opts, alias, ch.0 + range.start(), v.0, v.1)?;
                        }
                    }
                }
            }
        } else {
            // print entire book!
            for ch in book.chapters.iter().enumerate() {
                for v in ch.1.iter().enumerate() {
                    write_verse(&mut out, &opts, alias, ch.0, v.0, v.1)?;
                }
            }
        }

        Ok(out)
    }
}

fn write_verse(
    out: &mut String,
    opts: &Options,
    book: &str,
    chapter_num: usize,
    verse_num: usize,
    verse: &str,
) -> Result<(), std::fmt::Error> {
    let s = fill(
        &format!(
            "{:<width$} {}",
            format!("[{} {}:{}]", book, chapter_num + 1, verse_num + 1),
            verse,
            width = book.len() + 8
        ),
        opts,
    );
    writeln!(out, "{}", s)?;
    Ok(())
}
