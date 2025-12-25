use std::io::{stdin};

pub fn main() {
    let mut input : String  = String::new();

    println!("num facto");

    stdin().read_line(&mut input).expect("error reading input");

    let mut n  = input.trim().parse::<i32>().expect("error parsing num");


    if n ==0 {n = 1}
    for i in 2..n {
        n *= i;
    }

    println!("facto of {} = {}", input , n);

}