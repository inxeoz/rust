use std::io::{stdin, stdout};

fn main() {

    let mut input : String  = String::new();

    stdin().read_line(&mut input).expect("did not get input");

    let n : i32 = input.trim().parse::<i32> ().expect("invalid input");

    let mut sum : i32 = 0;

    for v in 1..=n {

        sum += v;

        println!("{}", v)
    }

    println!("sum {}", sum)
}