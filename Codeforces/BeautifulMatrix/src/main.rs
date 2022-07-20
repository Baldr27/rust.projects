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
    let mut m: Vec<Vec<i32>> = vec![];
    let mut x = 0;
    let mut y = 0;
    for _ in 0..5{
        let n = read_vec::<i32>();
        m.push(n);
    }
    for (i, vec) in m.iter().enumerate() {
        if vec.contains(&1i32){
            let temp: i32 = vec.iter().position(|&x| x == 1).unwrap().try_into().unwrap();
            x = (2-i as i32).abs();
            y = (2-temp).abs();
        }
    }

    println!("{}", x + y);

}
