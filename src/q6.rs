

use std::io::{stdin, stdout};
pub fn main() {
    let mut input  = String::new();

    println!("enter num");

    stdin().read_line(&mut input).expect("Failed to read line");

    let n : i32 = input.trim().parse::<i32>().expect("Not a number");

    (&mut input).clear();
    println!("enter pow");
    stdin().read_line(&mut input).expect("Failed to read line");

    let p = input.trim().parse::<i32>().expect("Not a number");


    fn pow(mut n: i32, p:i32 ) -> i32
   {

       let mut res = n;

       for _ in 0..p-1 {

           res *= n;
       }
     res

        };


    println!("{} ** {} = {}", n, p, pow(n, p));




}
