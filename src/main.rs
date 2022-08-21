use crate::args::{Args, Error};
use gumdrop::Options;

include!(concat!(env!("OUT_DIR"), "/rsb.rs"));

mod args;
mod display;

fn main() -> Result<(), Error> {
    let args = Args::parse_args_default_or_exit();

    if let Some(book) = args.book {
        match String::try_from(book) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
