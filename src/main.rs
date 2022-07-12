use rand::Rng;
use std::collections::HashMap;

const NUM_OF_LANGS: u8 = 11;
const TOTAL_TRIES: u8 = 6; // Head(1), Body(1), Feet(2), Hands(2)

fn main() {
    let languages = add_to_map();
    let lang = choose_lang(languages);

    println!("{}", lang);
}

fn add_to_map() -> HashMap<String, String> {
    let mut languages = HashMap::new();
    languages.insert("Java".to_string(), "System.out.println(\"Hello world!\");".to_string());
    languages.insert("Rust".to_string(), "println!(\"Hello world!\");".to_string());
    languages.insert("Python".to_string(), "print(\"Hello, world!\")".to_string());
    languages.insert("JavaScript".to_string(), "console.log(\"Hello, world!\");".to_string());
    languages.insert("Kotlin".to_string(), "println(\"Hello, world!\");".to_string());
    languages.insert("HTML".to_string(), "<h1>Hello world!</h1>".to_string());

    languages
}

fn choose_lang(languages: HashMap<String, String>) -> String {
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..NUM_OF_LANGS);

    let mut counter = 0;
    for (key, value) in &languages {
        if random_num == counter {
            key.to_string()
        }
        counter += 1;
    }
    panic!("Language failed to be chosen.");
}
