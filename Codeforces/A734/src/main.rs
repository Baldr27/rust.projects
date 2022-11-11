use std::io::prelude::*;

fn read<T>() -> T where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug, {
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
    let n = read::<usize>();
    let mut a = read::<String>().chars().collect::<Vec<char>>();
    let anton = a.iter().filter(|&x| *x == 'A').count();
    let danik = a.iter().filter(|&x| *x == 'D').count();

    if anton > danik {
        println!("Anton");
    } else if anton < danik {
        println!("Danik");
    } else {
        println!("Friendship");
    }
}
