pub fn main() {
    const list: [i32; 5] = [4, 2, 5, 8, 0];

    let value = 5;

    for (i, v) in list.iter().enumerate() {
        if *v == value {
            println!("index {i}");
            return;
        }
    }

    println!("index -1");
}
