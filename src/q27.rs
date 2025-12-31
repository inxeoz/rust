
use std::io::stdin;

pub fn main() {

    let mut input:String = String::new();
    println!("enter string");

    stdin().read_line(&mut input).expect("cant read");

    input  = input.trim().to_string();

    let mut chars:Vec<char> = Vec::new();

    for i in 0..input.len() {

        if  let Some(char) = input.chars().nth(i) {

            if ! chars.contains(&char) {
                chars.push(char);
            }
        }else { break; }



    }


    let unstr:String = chars.into_iter().collect();


    println!("unique string {unstr}");





}