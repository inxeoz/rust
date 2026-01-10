pub fn main() {
    let mut arr = [2, 0, 90, 2, 34, 78, 2, 84, 23, 0];
    let len = arr.len();
    perform_quick_sort(0, len - 1, &mut arr);
    println!("sorted {:?}", arr);
}

pub fn perform_quick_sort(start: usize, end: usize, arr: &mut [i32]) {
    if start >= end {
        return;
    }

    let pivot = arr[end];
    let mut i = start;

    for j in start..end {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    // Move pivot to its final position
    arr.swap(i, end);

    // Recurse on partitions
    if i > 0 {
        perform_quick_sort(start, i - 1, arr);
    }
    perform_quick_sort(i + 1, end, arr);
}
