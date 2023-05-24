use clap::Parser;

use super::day::Day;

/// Simple program to spit out a random TGIF acronym
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The day of the week you're thankful for
    #[arg(short, long, default_value_t=Day::Friday)]
    pub day: Day,
}
