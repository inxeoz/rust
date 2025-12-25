
pub fn main() {

    let mut arr = [4, 5, 1, 2, 0];

    let mut left = 0;
    let mut right  = arr.len() - 1;

    println!("original arr {:?} ", &arr);

    while left != right && left < right {

        let left_v = arr[left];
        let right_v = arr[right];

        arr[left] = right_v;
        arr[right] = left_v;

        left += 1;
        right -= 1;

    }

    println!("original arr {:?} ", &arr);

}