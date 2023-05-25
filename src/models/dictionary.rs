use std::{collections::HashMap, fmt, fs};

use clap::ValueEnum;

#[derive(Debug)]
pub struct Dictionary {
    pub words: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum DictionaryType {
    All,
    Common,
}

impl fmt::Display for DictionaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DictionaryType::All => write!(f, "all"),
            DictionaryType::Common => write!(f, "common"),
        }
    }
}

impl Dictionary {
    /// Creates a new instance of the dictionary struct
    pub fn new(t: DictionaryType, cached: bool) -> Self {
        match t {
            DictionaryType::All => {
                let all_words = Dictionary::from_dict_file();

                Dictionary { words: all_words }
            }
            DictionaryType::Common => {
                let src = if cached {
                    "./src/common_words.txt"
                } else {
                    "./src/war_and_peace.txt"
                };

                let min = if cached { 0 } else { 25 };

                let common_words = Dictionary::from_src(src, &min);

                Dictionary {
                    words: common_words,
                }
            }
        }
    }

    /// Returns of a vector of the words in the dict file
    fn from_dict_file() -> Vec<String> {
        fs::read_to_string("/usr/share/dict/words")
            .expect("Missing dict file")
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    /// Uses the words from a given src file
    fn from_src(src: &str, min: &i32) -> Vec<String> {
        let all_words = fs::read_to_string(src)
            .expect("Could not read book")
            .split(" ")
            .map(|s| s.trim().to_lowercase().to_string())
            .collect::<Vec<String>>();

        let mut map: HashMap<String, i32> = HashMap::new();

        for word in all_words {
            if let Some(count) = map.get(&word) {
                map.insert(word, count + 1);
            } else {
                map.insert(word, 0);
            }
        }

        map.iter()
            .filter(|(_, &count)| count.ge(&min))
            .map(|(word, _)| word.to_string())
            .collect::<Vec<String>>()
    }
}
