pub fn main() {
    let mut arr: [i32; 10] = [2, 0, 90, 2, 34, 78, 2, 84, 23, 0];
    let n = arr.len();

    for _ in 0..n - 1 {
        let mut swap_happen = false;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swap_happen = true;
            }
        }

        if !swap_happen {
            break;
        }
    }

    println!("sorted {:?}", arr);
}
