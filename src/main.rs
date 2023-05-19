use rand::seq::SliceRandom;
use std::fs;

const MAX: usize = 7;
const MIN: usize = 3;

fn main() {
    let contents = fs::read_to_string("/usr/share/dict/words").expect("Missing dict file");

    let all_words = contents.split("\n").collect::<Vec<&str>>();

    let t_words = filter_words(&all_words, 't');
    let g_words = filter_words(&all_words, 'g');
    let i_words = filter_words(&all_words, 'i');
    let f_words = filter_words(&all_words, 'f');

    let t_word = pick_random(t_words);
    let g_word = pick_random(g_words);
    let i_word = pick_random(i_words);
    let f_word = pick_random(f_words);

    println!("{0} {1} {2} {3}", t_word, g_word, i_word, f_word);
}

fn pick_random(vec: Vec<&str>) -> &str {
    let mut rng = rand::thread_rng();

    vec.choose(&mut rng).unwrap()
}

fn filter_words<'a>(vec: &'a Vec<&'a str>, letter: char) -> Vec<&'a str> {
    vec.iter()
        .filter(|w| w.starts_with(letter) && w.len().ge(&MIN) && w.len().le(&MAX))
        .cloned()
        .collect()
}
