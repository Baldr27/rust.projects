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
    let s: String = read_input::<String>().to_lowercase();
    let no_vowels: String = s.replace(['a','e','i','o','u','y'],"");
    for c in no_vowels.chars(){
        print!(".{}",c);
    }
}
