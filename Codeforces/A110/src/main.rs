use std::io::prelude::*;

fn main() {
    let input = read_input::<u64>();
    let n = numToVector(input);
    println!("{}", if isNearLucky(countLuckyN(n)) { "YES" } else { "NO" });
}

fn numToVector(a: u64)-> Vec<u32>{
    a.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn countLuckyN(a: Vec<u32>)-> i32{
    let mut count = 0;
    for i in a{
        if i == 4 || i == 7{
            count += 1;
        }
    }
    return count;
}

fn isNearLucky(a: i32)-> bool{
    let mut count = 0;
    for i in 0..a.to_string().len(){
        if a.to_string().chars().nth(i).unwrap() == '4' || a.to_string().chars().nth(i).unwrap() == '7'{
            count += 1;
        }
    }
    if count == a.to_string().len(){
        return true;
    }else{
        return false;
    }
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
