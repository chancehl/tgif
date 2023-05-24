use std::fmt;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Day::Monday => write!(f, "monday"),
            Day::Tuesday => write!(f, "tuesday"),
            Day::Wednesday => write!(f, "wednesday"),
            Day::Thursday => write!(f, "thursday"),
            Day::Friday => write!(f, "friday"),
            Day::Saturday => write!(f, "saturday"),
            Day::Sunday => write!(f, "sunday"),
        }
    }
}

impl Day {
    pub fn to_char(self) -> char {
        match self {
            Day::Monday => 'm',
            Day::Tuesday => 't',
            Day::Wednesday => 'w',
            Day::Thursday => 't',
            Day::Friday => 'f',
            Day::Saturday => 's',
            Day::Sunday => 's',
        }
    }
}
