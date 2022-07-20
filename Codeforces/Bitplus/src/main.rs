use std::io::prelude::*;

fn main() {
    let n = read_num::<usize>();
    let mut ans = 0;
    for _i in 0..n {
        let s = read_num::<String>();
        if s.contains("+") {
            ans += 1;
        }
        if s.contains("-") {
            ans -= 1;
        }
    }

    println!("{}", ans);
}

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