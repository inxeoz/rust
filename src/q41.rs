pub fn main() {
    let arr = [1, 2, 3, 4];

    for size in 1..=arr.len() {
        let mut j = 0;
        while j + size <= arr.len() {
            let set = &arr[j..(j + size)];

            print!("{:?} ", set);

            j += 1;
        }
    }
}
