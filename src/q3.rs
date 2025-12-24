
use std :: io :: { stdin};

pub fn main() {

    let mut input : String = String::new();

    stdin().read_line(&mut input).expect("error reading from stdin");

    let mut n : i32  = input.trim().parse::<i32>().expect("not i32");


    //  128

    // 128 / 10 = 12
    // 128 % 10 = 8 rev = 8

    // 12 / 10 = 1
    // 12 % 10 = 2 rev = 8 * 10 + 2

    // 1 / 10 = 0
    // 1 % 10 = 1 rev  =  ( 8 * 10 + 2 ) * 10 + 1



    let mut d = n % 10;
    let mut rev : i32 =d ;


    while n - d != 0  {

        n -= d;
        n /= 10;
        d = n % 10;
        rev = rev * 10  + d;

    }

    println!("{} ? {}", n, rev);



}