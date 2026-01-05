
use std::io::stdin;
use crate::q3::rev;

pub fn main() {
   let mut input  = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let n : i32 = input.trim().parse::<i32>().expect("Not a number");

    let rev = rev(n);

    if rev == n {
        println!("palindrome")
    }else {
        println!("not palindrome")
    }

}