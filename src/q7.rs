

use std::io::{stdin};

pub fn main() {
    let mut input: String = String::new();

    println!("iteration ");

    stdin().read_line(&mut input).expect("failed to read line");

    let n : u32 = input.trim().parse::<u32>().expect("not u32");


    let mut a = 0;
    let mut b = 1;

    print!("{} ", a);

    for _ in 1..n {

        print!("{} ", b);

        b = a + b;
        a = b - a;


    }

}