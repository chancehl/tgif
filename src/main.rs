use clap::Parser;
use models::args::Args;
use models::dictionary::Dictionary;
use rand::seq::SliceRandom;

mod models;

fn main() {
    let args = Args::parse();

    let dict_type = args.dict_type;
    let cached = args.cached;
    let day = args.day;

    for _ in 0..args.count {
        let dictionary = Dictionary::new(&dict_type, &cached);

        let t_words = filter_words(&dictionary.words, &'t');
        let g_words = filter_words(&dictionary.words, &'g');
        let i_words = filter_words(&dictionary.words, &'i');
        let day_words = filter_words(&dictionary.words, &day.clone().to_char());

        let t_word = pick_random(&t_words);
        let g_word = pick_random(&g_words);
        let i_word = pick_random(&i_words);
        let day_word = pick_random(&day_words);

        println!("{0} {1} {2} {3}", t_word, g_word, i_word, day_word);
    }
}

fn pick_random(vec: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();

    vec.choose(&mut rng).unwrap().to_string()
}

fn filter_words(vec: &Vec<String>, letter: &char) -> Vec<String> {
    vec.iter()
        .filter(|w| w.starts_with(*letter))
        .cloned()
        .collect()
}
