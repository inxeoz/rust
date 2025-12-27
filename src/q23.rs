
use std::io::{stdin};

pub fn main() {

    let mut input:String = String::new();

    stdin().read_line(&mut input).expect("err reading");

    input = input.trim().to_lowercase().to_string();

    let mut cc = 0;
    let mut cv = 0;

    const V : [u8; 5] = [1, 5, 9, 15, 21];

    for  c in input.chars() {
        let ch= c as u8 + 1- 'a' as u8;

        if ch >= 1 && ch <= 26  {

           if V.contains(&ch)  {
                cv += 1;
            }else {
               cc += 1;
           }
        }
    }

    println!("count v {cv}");

    println!("count c {cc}");
}