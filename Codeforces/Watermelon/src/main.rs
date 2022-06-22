use std::io::stdin;
 
fn main(){
    let mut input= String::new();
    stdin().read_line(&mut input).expect("failed to readline");
    let w:i32 = input.trim().parse().expect("no integer entered");
 
    if w==2{
        println!("NO");
    }else if w%2==0{
        println!("YES");
    }else{
        println!("NO");
    }
}