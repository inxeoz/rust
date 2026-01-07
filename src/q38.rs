pub fn main() {
    let mut arr: [i32; 10] = [2, 0, 90, 2, 34, 78, 2, 84, 23, 0];

    for i in 1..arr.len() {
        let mut current_pos = i;

        while current_pos > 0 && arr[current_pos - 1] > arr[current_pos] {
            arr.swap(current_pos, current_pos - 1);
            current_pos -= 1;
        }
    }

    println!("sorted {:?}", arr);
}
