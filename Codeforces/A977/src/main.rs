use std::io::prelude::*;

fn read_vec<T>() -> Vec<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug, {
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

fn main() {
    let input = read_vec::<u64>();
    let mut num = input[0];
    let nsub = input[1];
    for _ in 0..nsub {
        num = substract(num);
    }
    println!("{}", num);
}

fn substract(num: u64)-> u64{
    if num%10 == 0 {
        num/10
    }else{
        num-1
    }
}
