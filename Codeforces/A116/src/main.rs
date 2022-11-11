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

fn read_input<T>() -> T where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug, {
        std::io::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .parse()::<T>()
            .unwrap()
    }

fn main() {
    let n: i32 = read_input();
    let mut max_capacity: i32 = 0;
    let mut current_capacity: i32 = 0;

    for _ in 0..n{
        let n_stop: Vec<i32> = read_vec();
        current_capacity = current_capacity - n_stop[0] + n_stop[1];
        if current_capacity > max_capacity {
            max_capacity = current_capacity;
        }
    }

    println!("{}", max_capacity);
}
