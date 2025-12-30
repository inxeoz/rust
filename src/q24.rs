
use std::io::{stdin};


pub fn main() {

    let mut input  = String::new();

    println!("enter input");

    stdin().read_line(&mut input).expect("cant read");

    input = input.trim().to_string();


    for i in 0..input.len() {
        let chr1 = input.chars().nth(i).expect("cant get first char");

        let mut count  = 0;
        for j in 0..input.len() {
            let chr2 = input.chars().nth(j).expect("cant get second char");

            if chr2 == chr1 {

                count += 1;
                if count > 1 {
                    break;
                }
            }
        }

        if count == 1 {
            println!("first non - repeating char {chr1}");
            return;
        }
    }

    println!("every char repeats")


}