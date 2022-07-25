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

fn main() {
    let input = read_input::<String>();
    let mut together = 1;

    for i in 1..input.len() {
        if together >= 7 {
            println!("YES");
            return;
        }else{
            if input.chars().nth(i - 1) == input.chars().nth(i) {
                together+=1;
            }else{
                together = 1;
            }
        }
    }
    if together >= 7 {
        println!("YES");
    }else{
        println!("NO");
    }
}
