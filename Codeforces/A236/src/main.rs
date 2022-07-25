use std::io::prelude::*;

fn read_vec<T>() -> Vec<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug, {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split("")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let nick = read_vec::<String>();
    let mut unique = nick.clone();
    unique.sort();
    unique.dedup();
    if (unique.len()-1)%2 == 0 {
        println!("CHAT WITH HER!");
    }else{
        println!("IGNORE HIM!");
    }
}
