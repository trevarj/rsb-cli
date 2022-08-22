use std::ops::RangeInclusive;
use std::str::FromStr;

use gumdrop::Options;

/// Read the Russian Synodal Bible from the command-line
#[derive(Debug, Options)]
pub struct Args {
    /// prints help message
    #[options()]
    help: bool,

    #[options(command)]
    pub book: Option<Book>,
}

macro_rules! book_command {
    ($($num:literal, $alias:ident, $title:literal),*) => {
        #[derive(Debug, Options)]
        pub enum Book {
            $(
                #[options(help = $title)]
                $alias(ChapterVerseArg)
            ),*
        }

        impl Book {
            pub fn alias_num(&self) -> (&str, usize) {
                match self {
                    $(
                        Self::$alias(..) => (stringify!($alias), $num)
                    ),*
                }
            }

            pub fn chapter(&self) -> &Option<Chapter> {
                match self {
                    $(
                        Self::$alias(arg) => &arg.chapter_verse
                    ),*
                }
            }
        }
    };
}

#[derive(Debug, Options)]
pub struct ChapterVerseArg {
    /// print help message
    #[options()]
    help: bool,

    /// specify a chapter and verse(s) [ch<:v><-v>]
    #[options(free)]
    pub chapter_verse: Option<Chapter>,
}

#[derive(Debug)]
pub enum Chapter {
    Single {
        chapter: usize,
        verses: Option<Verses>,
    },
    Range(RangeInclusive<usize>),
}

impl FromStr for Chapter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(':') {
            Some((ch, verses)) => Self::Single {
                chapter: parse_number(ch)?,
                verses: Some(Verses::from_str(verses)?),
            },
            None => match s.split_once('-') {
                Some((low, high)) => {
                    let low = parse_number(low)?;
                    let high = parse_number(high)?;
                    Self::Range(low..=high)
                }
                None => Self::Single {
                    chapter: parse_number(s)?,
                    verses: None,
                },
            },
        })
    }
}

#[derive(Debug)]
pub enum Verses {
    Single(usize),
    Range(RangeInclusive<usize>),
}

impl FromStr for Verses {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once('-') {
            Some((low, high)) => match (parse_number(low), parse_number(high)) {
                (Ok(low), Ok(high)) if low < high => Verses::Range(low..=high),
                _ => {
                    return Err(Error::InvalidVerseRange(s.to_string()));
                }
            },
            None => {
                let single = parse_number(s)?;
                Verses::Single(single)
            }
        })
    }
}

fn parse_number(input: &str) -> Result<usize, Error> {
    input
        .parse::<usize>()
        .map(|n| {
            n.checked_sub(1)
                .ok_or_else(|| Error::InvalidInput(n.to_string()))
        })
        .map_err(|_| Error::InvalidInput(input.to_string()))?
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid chapter {} of book {title} ({chapters} chapters).", num + 1)]
    InvalidChapter {
        num: usize,
        title: String,
        chapters: usize,
    },
    #[error("Invalid verse {} of book {title}, chapter {chapter} ({verses} verses).", num + 1)]
    InvalidVerse {
        num: usize,
        title: String,
        chapter: usize,
        verses: usize,
    },
    #[error("Chapter or verse provided is not valid: {0}")]
    InvalidInput(String),
    #[error("Invalid verse range: {0}")]
    InvalidVerseRange(String),
    #[error("Formatting error")]
    Fmt(#[from] std::fmt::Error),
}

book_command! {
    0, Gen , "Бытие",
    1, Ex, "Исход",
    2, Lev, "Левит",
    3, Num, "Числа",
    4, Deut, "Второзаконие",
    5, Josh, "Книга Иисуса Навина",
    6, Judg, "Книга Судей израилевых",
    7, Rth, "Книга Руфи",
    8, King1, "Первая книга Царств",
    9, King2, "Вторая книга Царств",
    10, King3, "Третья книга Царств",
    11, King4, "Четвертая книга Царств",
    12, Chron1, "Первая книга Паралипоменон",
    13, Chron2, "Вторая книга Паралипоменон",
    14, Ezr1, "Первая книга Ездры",
    15, Nehem, "Книга Неемии",
    16, Ezr2, "Вторая книга Ездры",
    17, Tov, "Книга Товита",
    18, Judf, "Книга Иудифи",
    19, Est, "Книга Есфири",
    20, Job, "Книга Иова",
    21, Psal, "Псалтирь",
    22, Prov, "Притчи Соломона",
    23, Eccl, "Книга Екклезиаста",
    24, Song, "Песнь песней Соломона",
    25, Solom, "Книга Премудрости Соломона",
    26, Sir, "Книга Премудрости Иисуса, сына Сирахова",
    27, Isiah, "Книга пророка Исаии",
    28, Jer, "Книга пророка Иеремии",
    29, Lam, "Плач Иеремии",
    30, PJer, "Послание Иеремии",
    31, Bar, "Книга пророка Варуха",
    32, Ezek, "Книга пророка Иезекииля",
    33, Dan, "Книга пророка Даниила",
    34, Hos, "Книга пророка Осии",
    35, Joel, "Книга пророка Иоиля",
    36, Amos, "Книга пророка Амоса",
    37, Avd, "Книга пророка Авдия",
    38, Jonah, "Книга пророка Ионы",
    39, Mic, "Книга пророка Михея",
    40, Naum, "Книга пророка Наума",
    41, Habak, "Книга пророка Аввакума",
    42, Sofon, "Книга пророка Софонии",
    43, Hag, "Книга пророка Аггея",
    44, Zah, "Книга пророка Захарии",
    45, Mal, "Книга пророка Малахии",
    46, Mac1, "Первая книга Маккавейская",
    47, Mac2, "Вторая книга Маккавейская",
    48, Mac3, "Третья книга Маккавейская",
    49, Ezr3, "Третья книга Ездры",
    50, Matt, "От Матфея святое благовествование",
    51, Mark, "От Марка святое благовествование",
    52, Luke, "От Луки святое благовествование",
    53, John, "От Иоанна святое благовествование",
    54, Acts, "Деяния святых апостолов",
    55, James, "Соборное послание святого апостола Иакова",
    56, Pete1, "Первое соборное послание святого апостола Петра",
    57, Pete2, "Второе соборное послание святого апостола Петра",
    58, John1, "Первое соборное послание святого апостола Иоанна",
    59, John2, "Второе соборное послание святого апостола Иоанна",
    60, John3, "Третье соборное послание святого апостола Иоанна",
    61, Jude, "Соборное послание святого апостола Иуды",
    62, Romans, "Послание к Римлянам святого апостола Павла",
    63, Cor1, "Первое послание к Коринфянам святого апостола Павла",
    64, Cor2, "Второе послание к Коринфянам святого апостола Павла",
    65, Gal, "Послание к Галатам святого апостола Павла",
    66, Eph, "Послание к Ефесянам святого апостола Павла",
    67, Phil, "Послание к Филиппийцам святого апостола Павла",
    68, Col, "Послание к Колоссянам святого апостола Павла",
    69, Thes1, "Первое послание к Фессалоникийцам (Солунянам) святого апостола Павла",
    70, Thes2, "Второе послание к Фессалоникийцам (Солунянам) святого апостола Павла",
    71, Tim1, "Первое послание к Тимофею святого апостола Павла",
    72, Tim2, "Второе послание к Тимофею святого апостола Павла",
    73, Titus, "Послание к Титу святого апостола Павла",
    74, Phlm, "Послание к Филимону святого апостола Павла",
    75, Hebr, "Послание к Евреям святого апостола Павла",
    76, Rev, "Откровение святого Иоанна Богослова"
}
