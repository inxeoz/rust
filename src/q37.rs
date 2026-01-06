pub fn main() {
    let mut arr: [i32; 10] = [2, 0, 90, 2, 34, 78, 2, 84, 23, 0];

    for i in 0..arr.len() - 1 {
        let mut swap_pos = i;

        for next_min_pos in (i + 1)..arr.len() {
            if arr[next_min_pos] < arr[swap_pos] {
                swap_pos = next_min_pos;
            }
        }

        if swap_pos > i {
            arr.swap(swap_pos, i);
        }
    }

    println!("sorted {:?} ", arr);
}
