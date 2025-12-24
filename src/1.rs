use std::io::{stdin, stdout};

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("did not get it");
    let n = input.trim().parse::<i32>().expect("invalid input");
}
