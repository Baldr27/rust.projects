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
        .split('+')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let mut nums: Vec<i32> = read_vec();
    nums.sort();
    let mut ans = nums[0].to_string();
    for i in 1..nums.len(){
        ans+=&format!("+{}",nums[i]);
    }
    println!("{}",ans);
}
