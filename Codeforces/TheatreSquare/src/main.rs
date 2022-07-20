use std::io::BufRead;

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
    let input: Vec<i64> = read_vec();
    let ans = (input[0] as f64/input[2] as f64).ceil() as i64 * (input[1] as f64/input[2] as f64).ceil() as i64;
    println!("{}", ans);
}
