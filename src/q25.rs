

use std::io::stdin;
use std::collections::HashMap;
pub fn main() {

    let mut input:String = String::new();

    println!("first word");

    stdin().read_line(&mut input).expect("cant read first word");
    let a = input.trim().to_string();
    input.clear();
    println!("sec word");
    stdin().read_line(&mut input).expect("cant read sec word");


    let b = input.trim().to_string();
    input.clear();


    println!("is anagram {} {} \n {}", &a, &b, is_anagram(&a, &b) );


}


fn is_anagram(a:&str, b:&str) -> bool{


    if a.len() != b.len() {
        return  false;
    }


    let mut count : HashMap<char, usize> = HashMap::new();


    for ch in a.chars()
    {

        *count.entry(ch).or_insert(0) += 1;
    }


    for ch in b.chars() {


        match count.get_mut(&ch) {

            Some(c) => {

                if *c == 0 {

                    return false;
                }
                *c -=1;
            },
            _=> return false


        }
    }

 return true;

}