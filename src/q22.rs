use std::io::{stdin};


pub fn main() {

    let mut input:String = String::new();

    println!("enter string");

    stdin().read_line(&mut input).expect("err reading");

    input =  input.trim().to_string();

    let len = input.len();

    if len <= 0 {
        println!("not string");
        return

    }

    let mid = len / 2 ;

    for i  in 0..mid {

        let left = input.chars().nth(i).expect("err reading it");

        let right = input.chars().nth(len - i - 1).expect("err reading it");

        if left != right {

            println!("not palindrome");
            return;
        }

    }


    println!("palindrome");





}