use std::io::{stdin};

pub fn main() {

    let mut input: String = String::new();

    println!("enter iteration");

    stdin().read_line(&mut input).expect("cant get number");

    let n = input.trim().parse::<u32>().expect("cant parse");

    let a = 0;
    let b = 1;
    print!("{} ", a);
    next_fibo(a, b, n);

}


fn next_fibo(a:u32, b:u32, iter: u32) {
    print!("{} ", b);

    if iter -1 <= 1 {
        return;
    }
    next_fibo(b, a + b, iter-1);
}