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
    let mut input = read_input::<i32>();
    let mut s = 1;

    while input>5 {
        input-=5;
        s+=1;
    }
    println!("{}", s);
}
