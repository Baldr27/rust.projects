use std::io::prelude::*;

fn main() {
    let input = read_vec::<i32>();
    let m = input[0];
    let n = input[1];

    println!("{}", m * n / 2);
}

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}