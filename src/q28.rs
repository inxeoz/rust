use std::io::stdin;

pub fn main() {
    let mut input: String = String::new();
    println!("enter string");

    stdin().read_line(&mut input).expect("cant read");
    input = input.trim().to_string();

    let pairs = ["{}", "[]", "()"];

    for _ in 0..input.len() / 2 {
        let before = input.len();
        for pair in pairs {
            input = input.replace(pair, "");
        }
        if before == input.len() {
            break;
        }
    }

    if input.len() > 0 {
        println!("invalid");
    } else {
        println!("valid")
    }
}
