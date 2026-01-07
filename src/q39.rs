pub fn main() {
    let mut arr: [i32; 10] = [2, 0, 90, 2, 34, 78, 2, 84, 23, 0];

    let arrc = perform_merge_sort(&arr);

    println!("sorted {:?}", arrc);
}

fn perform_merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;

    let arr1 = perform_merge_sort(&arr[..mid]);
    let arr2 = perform_merge_sort(&arr[mid..]);

    return merge_two_sorted(&arr1, &arr2);
}
fn merge_two_sorted(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut new_arr = Vec::with_capacity(arr1.len() + arr2.len());

    let mut arr1_i = 0;
    let mut arr2_i = 0;

    while arr1_i < arr1.len() && arr2_i < arr2.len() {
        if arr1[arr1_i] <= arr2[arr2_i] {
            new_arr.push(arr1[arr1_i]);
            arr1_i += 1;
        } else {
            new_arr.push(arr2[arr2_i]);
            arr2_i += 1;
        }
    }

    new_arr.extend_from_slice(&arr1[arr1_i..]);
    new_arr.extend_from_slice(&arr2[arr2_i..]);

    return new_arr;
}
