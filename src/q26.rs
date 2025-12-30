use std::cmp::min;
use std::io::{stdin};


pub fn main() {

    let mut input:String = String::new();
    println!("enter list of words seperated by space");
    // geeksforgeeks geeks geek geezer

    stdin().read_line(&mut input).expect("cant read");
    input = input.trim().to_string();

    let mut list: Vec<&str> = input.split_whitespace().collect();

    list.sort_by_key(|w| w.len());

    let mut min_word = list[0].to_string();

    for word in list.iter().skip(1) {
        min_word = longest_prefix(&min_word, word);

    }


    println!("longest prefix {min_word}");



}


fn longest_prefix(a:&str, b:&str) -> String {
    let b_chars = b.chars();
    let a_chars = a.chars();

    let mut btye_len  = 0;

    for (ca, cb) in a_chars.zip(b_chars) {

        if ca == cb {
            btye_len += ca.len_utf8();
        }else {
            break;
        }
    }

    a[..btye_len].to_string()

}