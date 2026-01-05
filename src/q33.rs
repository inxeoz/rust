pub fn main() {
    const ARR: [i32; 10] = [2, 4, 5, 8, 8, 9, 12, 34, 90, 90];

    search(ARR, 0, (ARR.len() - 1) as i32, 100);
}

fn search(list: [i32; 10], mut left: i32, mut right: i32, finding: i32) -> i32 {
    if !(left < right) {
        println!("-1 not found");
        return -1;
    }

    let mid: i32 = left + (right - left) / 2;

    let value = list[mid as usize];

    if value == finding {
        println!("found at {mid}");
        return mid;
    }

    if finding > value {
        left = mid;
    } else {
        right = mid;
    }

    return search(list, left, right, finding);
}
