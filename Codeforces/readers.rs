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

<<<<<<< HEAD
fn read_vec<T>() -> Vec<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug,{
=======
fn read_vec<T>() -> Vec<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug, {
>>>>>>> e540a7612669e7b8ca21960061d18db7b9f60484
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