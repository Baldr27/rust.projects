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

fn read_vec<T>() -> Vec<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug,{
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
    let n = read_input::<usize>();
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for _ in 0..n {
        let v = read_vec::<i32>();
        x += v[0];
        y += v[1];
        z += v[2];
    }
    if x == 0 && y == 0 && z == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
