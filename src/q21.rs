use std::io::{stdin};


pub fn main() {

    let mut input : String = String::new();

    println!("enter str");

    stdin().read_line(&mut input).expect("cant read");

    input = input.trim().to_string();

    println!("str {input}");


    let len = input.len() as u32;

    if len <= 1 {
        println!("input {input}");
        return;

    }

    let index  =  len / 2  - 1 ;




    for i in 0..=index {
        let rev_i = len - ( i +1 ) ;


        let left = input.chars().nth(i as usize).expect(&format!("missing char at {i}") );

        let right = input.chars().nth(rev_i as usize).expect(&format!("missing char at {i}") );



        input.replace_range((rev_i as usize)..(rev_i as usize) +1 , &left.to_string());

        input.replace_range((i as usize) ..(i as usize) +1, &right.to_string());

    }




    println!("str {input}");





}