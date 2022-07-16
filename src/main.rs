use rand::Rng;
use std::collections::HashMap;
use std::io::stdin;

const NUM_OF_LANGUAGES: u8 = 6;
const TOTAL_TRIES: u8 = 6; // Head(1), Body(1), Feet(2), Hands(2)

fn main() {
    let languages = create_map();

    let lang = choose_lang(&languages);
    let hello_world = get_hello_world(&languages, &lang);
    let char_vec: Vec<char> = lang.chars().collect();

    println!("\nMake a guess...");
    print_vec(lang.len() as u8, &char_vec);
    println!();

    let guess: char = take_guess();

    if char_vec.contains(&guess) {
    } else {
    }
}

/// Creates a HashMap with programming languages, and their corresponding "Hello world!" statements.
fn create_map() -> HashMap<String, String> {
    let mut languages = HashMap::new();
    languages.insert(
        "JAVA".to_owned(),
        "System.out.println(\"Hello world!\");".to_owned(),
    );
    languages.insert("RUST".to_owned(), "println!(\"Hello world!\");".to_owned());
    languages.insert("PYTHON".to_owned(), "print(\"Hello, world!\")".to_owned());
    languages.insert(
        "JAVASCRIPT".to_owned(),
        "console.log(\"Hello, world!\");".to_owned(),
    );
    languages.insert(
        "KOTLIN".to_owned(),
        "println(\"Hello, world!\");".to_owned(),
    );
    languages.insert("HTML".to_owned(), "<h1>Hello world!</h1>".to_owned());

    languages
}

/// Randomly chooses a language from the HashMap.
fn choose_lang(languages: &HashMap<String, String>) -> String {
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..NUM_OF_LANGUAGES);

    let mut counter = 0;
    for (key, _value) in languages.iter() {
        if random_num == counter {
            return String::from(key);
        } else {
            counter += 1;
        }
    }
    panic!("Language failed to be chosen.");
}

fn get_hello_world(languages: &HashMap<String, String>, key: &str) -> String {
    let value = languages.get(key);

    match value {
        Some(val) => val.to_owned(),
        _ => panic!("Failed to get Hello World."),
    }
}

fn print_vec(amount: u8, vec: &Vec<char>) {
    println!("{:?}", vec);
}

fn take_guess() -> char {
    if TOTAL_TRIES <= 0 {
        println!("You have used all your tries.");
        return '!'.to_owned();
    }

    let mut guess = String::new();
    let _ = stdin()
        .read_line(&mut guess)
        .expect("Failed to read line")
        .to_string()
        .trim();

    guess.trim().to_owned().to_uppercase().parse().unwrap()
}
