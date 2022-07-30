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
    let ab = read_vec::<i32>();
    let mut a = ab[0];
    let mut b = ab[1];
    let mut ans = 0;
    while a<=b {
        a *= 3;
        b *= 2;
        ans += 1;
    }
    println!("{}", ans);
}
