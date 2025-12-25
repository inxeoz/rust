use std::io::{stdin};

pub fn main() {

    let mut input : String = String::new();

    println!("enter facto");

    stdin().read_line(&mut input).expect("err reading input");

    let mut n = input.trim().parse::<u32>().expect("error parsing num");

    println!("facto of {} is {}", n, facto(n));

}

fn facto( n : u32) -> u32 {

    if n <= 0 {
        return 1;
    }else{
        return n * facto(n-1);
    }
}