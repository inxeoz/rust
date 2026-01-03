use std::io::stdin;

pub fn main() {
    let mut input: String = String::new();

    println!("enter string");

    stdin().read_line(&mut input).expect("cant read");

    input = input.trim().to_string();

    let mut right = 0;

    let chars: Vec<char> = input.chars().collect();

    let mut set: Vec<char> = Vec::new();

    let mut longest_sub = String::new();
    let size = chars.len();

    while right < size {
        let ch = chars[right];
        if !set.contains(&ch) {
            (&mut set).push(ch);
            right += 1;
        } else {
            set.remove(0);
        }

        if longest_sub.chars().count() < set.len() {
            longest_sub = set.iter().collect::<String>();
        }
    }

    println!("longest substring {}", longest_sub);
}
