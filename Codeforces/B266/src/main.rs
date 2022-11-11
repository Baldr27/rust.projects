use std::io::prelude::*;

fn main() {
    let a = read_vec::<i32>();
    let n: i32 = a[0];
    let t: i32 = a[1];
    let mut queue = read_input::<String>();

    for _ in 0..t-1{
        for i in (0..n-1).step_by(2){
            if queue[i..i+2] == "BG".to_owned(){
                queue.replace_range(i..i+2, "GB");
            }
        }
    }

    println!("{}", queue);
}

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
