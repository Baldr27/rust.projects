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
    let n = read_input::<i32>();
    let mut stones = read_vec::<String>();
    let mut ans = 0;
    for c in 0..stones.len()-1{
        if stones.get(c).unwrap() == ""{
            stones.remove(c);
        }
    }
    for c in 0..stones.len()-1{
        if stones.get(c) == stones.get(c+1){
            ans+=1;
        }
    }
    println!("{}",ans);
}
