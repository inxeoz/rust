pub fn main() {
    const ARR: [i32; 10] = [2, 3, 4, 10, 2, 5, 0, 0, 90, 90];

    let mut first: i32 = -1;
    let mut last: i32 = -1;

    let occur = 2;

    for (i, v) in ARR.into_iter().enumerate() {
        if (v == occur) {
            if first == -1 {
                first = i as i32;
            } else {
                last = i as i32;
            }
        }
    }

    println!("{first} {last}");
}
