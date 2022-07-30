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
    let s = read_input::<String>();
    let mut up = 0;
    let mut low = 0;
    for c in s.chars() {
        if c.is_lowercase() {
            low += 1;
        } else {
            up += 1;
        }
    }
    if up > low {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}
