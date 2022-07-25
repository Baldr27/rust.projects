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
    let input = read_vec::<i32>();
    let k = input[0];
    let n = input[1];
    let w = input[2];

    let mut price = 0;

    for i in 1..=w{
        price += i*k;
    }

    if n-price < 0{
        println!("{}", (n-price).abs());
    }else{
        println!("0");
    }

}
