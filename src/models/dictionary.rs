use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Dictionary {
    pub words: Vec<String>,
}

pub enum DictionaryType {
    All,
    Common,
}

impl Dictionary {
    /// Creates a new instance of the dictionary struct
    pub fn new(t: DictionaryType) -> Self {
        match t {
            DictionaryType::All => {
                let all_words = Dictionary::from_dict_file();

                Dictionary { words: all_words }
            }
            DictionaryType::Common => {
                let common_words = Dictionary::from_war_and_peace();

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

    /// Uses the words from war and peace
    fn from_war_and_peace() -> Vec<String> {
        let all_words = fs::read_to_string("./src/war_and_peace.txt")
            .expect("Could not read book")
            .split(" ")
            .map(|s| s.to_string())
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
            .filter(|(_, &count)| count.gt(&25))
            .map(|(word, _)| word.to_string())
            .collect::<Vec<String>>()
    }
}
