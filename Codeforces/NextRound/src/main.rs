use std::io::prelude::*;

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

fn main() {
    let info = read_vec::<i32>();
    let _n = info[0];
    let k = info[1];
    let scores = read_vec::<i32>();
    let mut advanced = 0;
    for score in &scores{
        if score >= &scores[(k-1) as usize] && score != &0{
            advanced+=1;
        }
    }
    println!("{}", advanced);
}
