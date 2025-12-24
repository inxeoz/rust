use std::io::{stdin, stdout, };
pub fn main() {

    let mut input : String = String::new();

    stdin().read_line(&mut input).expect("err");

    let mut n : i32 = input.trim().parse::<i32>().expect("not i32");

    let mut count = 0;

    while n % 10 != 0 {
        count += 1;
        n = n - n % 10;
        n = n / 10;
    }

    println!("count {} ", count);

}