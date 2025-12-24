
use std::io::{stdin, stdout};

pub fn main() {

    let mut input : String  = String::new();

    stdin().read_line(&mut input).expect("failed to read line");

    let n : i32 = input.trim().parse::<i32>().expect("failed to parse integer");

    for v in 0..=n {

        if v % 2 == 0 {
            println!("{} is even ", v)
        }
    }


}