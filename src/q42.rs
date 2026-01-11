pub fn main() {
    let list = permute("inxeoz");
    println!("{:?} \ncount {}", list, list.len());
}

fn permute(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    permutation(chars)
}

fn permutation(chars: Vec<char>) -> Vec<String> {
    // Base case: one letter → one word
    if chars.len() <= 1 {
        return vec![chars.into_iter().collect()];
    }

    let mut result = Vec::new();

    // Pick each letter once
    for i in 0..chars.len() {
        let mut remaining = chars.clone();
        let picked = remaining.remove(i);

        // Permute the rest
        for word in permutation(remaining) {
            // Put picked letter in front
            let mut new_word = word;
            new_word.insert(0, picked);
            result.push(new_word);
        }
    }

    result
}
