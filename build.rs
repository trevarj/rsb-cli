use std::fs::File;
use std::io::Write;

use quote::__private::TokenStream;
use quote::quote;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=rsb/");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("rsb.rs");

    let dir = std::fs::read_dir("rsb/").expect("rsb source not found");
    let mut file = File::create(&dest_path).expect("error creating generated file");
    let mut books = vec![];
    for e in dir {
        let entry = e.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        // extract the book number as it exists in the file name
        // or continue if this is not a book file (ex. README)
        let book_num = match file_name
            .split_once('_')
            .and_then(|(n, _)| n.parse::<u32>().ok())
        {
            Some(num) => num,
            None => continue,
        };

        // println!(
        //     "cargo:warning=Parsing book file {}, number {}",
        //     file_name, book_num
        // );
        let book = std::fs::read_to_string(entry.path()).expect("file does not exist");

        let mut title = None;
        let mut chapters = vec![];
        let mut current_chapter = vec![];
        for line in book.lines() {
            if line.starts_with("===") {
                // found chapter line
                // if not first chapter, we found a new one
                if !current_chapter.is_empty() {
                    chapters.push(std::mem::take(&mut current_chapter));
                }
            } else if line.starts_with("==") {
                // found book title line
                title = Some(line.trim_matches('=').trim())
            } else if let Some((_num, verse)) = line.split_once(' ') {
                // found verse line
                current_chapter.push(verse.trim());
            }
        }
        // push final chapter
        chapters.push(current_chapter);

        let book = construct_book(title.expect("no book title"), chapters);
        books.push((book_num, book));
    }

    books.sort_by_key(|b| b.0);
    let books: Vec<TokenStream> = books.into_iter().map(|b| b.1).collect();

    println!("cargo:warning=Writing file {}", dest_path.display());
    let books_len = books.len();
    let module = quote! {
        pub struct Book {
            pub title: &'static str,
            pub chapters: &'static [&'static [&'static str]],
        }

        impl std::ops::Index<usize> for Book {
            type Output = &'static [&'static str];

            fn index(&self, index: usize) -> &Self::Output {
                &self.chapters[index]
            }
        }

        impl std::ops::Index<std::ops::Range<usize>> for Book {
            type Output = [&'static [&'static str]];

            fn index(&self, range: std::ops::Range<usize>) -> &Self::Output {
                &self.chapters[range]
            }
        }

        impl std::ops::Index<std::ops::RangeFull> for Book {
            type Output = [&'static [&'static str]];

            fn index(&self, range: std::ops::RangeFull) -> &Self::Output {
                &self.chapters[range]
            }
        }

        pub const BOOKS: [Book; #books_len] = [#(#books,)*];
    };
    file.write_all(module.to_string().as_bytes())
        .expect("error writing generated file");
}

fn construct_book(title: &str, chapters: Vec<Vec<&str>>) -> TokenStream {
    let chapters: Vec<TokenStream> = chapters
        .into_iter()
        .map(|verses| {
            quote! {
                &[#(#verses,)*]
            }
        })
        .collect();
    quote! {
        Book {
            title: #title,
            chapters: &[#(#chapters,)*]
        }
    }
}
