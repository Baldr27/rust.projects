use std::io::prelude::*;

fn read_num<T>() -> T
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
        .parse::<T>()
        .unwrap()
}

fn main() {
    let a = read_num::<String>().to_lowercase();
    let b = read_num::<String>().to_lowercase();
    
    if a == b {
        println!("0");
    }else{
        match a>b {
            true => println!("1"),
            false => println!("-1"),
        }
    }
}
