use clap::Parser;

use super::{day::Day, dictionary::DictionaryType};

/// Simple program to spit out a random TGIF acronym
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The day of the week you're thankful for
    #[arg(short, long, default_value_t=Day::Friday)]
    pub day: Day,

    /// The dictionary type to use
    #[arg(short = 't', long = "dictionary-type", default_value_t=DictionaryType::Common)]
    pub dict_type: DictionaryType,
}
