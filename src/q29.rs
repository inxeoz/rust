use std::io::stdin;

pub fn main() {
    let mut input: String = String::new();
    println!("enter string ");

    stdin().read_line(&mut input).expect("cant read");

    input = input.trim().to_string();

    let mut num: isize = 0;
    let mut neg_sign_assigned = false;

    let mut is_prev_digit = false;

    for (i, char) in input.chars().enumerate() {
        if i == 0 && (char == '-' || char == '+') {
            match char {
                '-' => {
                    neg_sign_assigned = true;
                    continue;
                }
                '+' => {
                    continue;
                }

                _ => {
                    break;
                }
            }
        }
        if char == '_' && is_prev_digit {
            is_prev_digit = false;
            continue;
        } else if let Some(d) = char.to_digit(10) {
            is_prev_digit = true;
            num = num * 10 + d as isize;
        } else {
            break;
        }
    }

    if neg_sign_assigned {
        num = -1 * num;
    }

    println!("My Num {} ", num);
}
