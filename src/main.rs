use dialoguer::console::Term;
use dialoguer::{FuzzySelect, MultiSelect};

use crate::theme::Theme;

include!(concat!(env!("OUT_DIR"), "/rsb.rs"));

mod theme;

fn main() {
    println!("Genesis 1:1 {}", BOOKS[0][0][0]);

    println!("Matthew 1 \n{}", BOOKS[51][0].join("\n"));

    let book = FuzzySelect::with_theme(&Theme::book())
        .with_prompt("Select a Book")
        .default(0)
        .items(&BOOKS.iter().map(|b| b.title).collect::<Vec<&str>>())
        .interact_on_opt(&Term::stdout())
        .unwrap()
        .unwrap()
        .0;

    let chapters: Vec<String> = (1..BOOKS[book][..].len())
        .map(|n| format!("Глава {}", n))
        .collect();

    let chapter = FuzzySelect::with_theme(&Theme::chapter())
        .with_prompt("Select a chapter")
        .default(0)
        .items(&chapters)
        .interact_on_opt(&Term::stdout())
        .unwrap()
        .unwrap()
        .0;

    let book_title = BOOKS[book].title;
    let verses = MultiSelect::with_theme(&Theme::verse())
        .with_prompt("Select a verse")
        .items(&BOOKS[book][chapter])
        .interact_on_opt(&Term::stdout())
        .unwrap()
        .unwrap();

    for v in verses {
        println!(
            "[{} {}:{}] {}",
            book_title,
            chapter + 1,
            v + 1,
            BOOKS[book][chapter][v]
        );
    }
}
