use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed");
    let n:i32 = input.trim().parse().expect("not integer");
    for i in 1..n{
        let mut sum:[i32; 3] = input.split_whitespace().parse().expect("no iteger");
    }
    println!
}
