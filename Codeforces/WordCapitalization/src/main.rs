use std::io::prelude::*;

fn read_input<T>() -> T where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug, {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<T>()
        .unwrap()
}

fn main() {
    let mut word = read_input::<String>();
    let first_letter: String = word.chars().next().unwrap().to_string().to_uppercase();
    word.remove(0);
    print!("{}", first_letter);
    for char in word.chars() {
        print!("{}", char);
    }
}
