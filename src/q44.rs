const SIZE: usize = 3;
use tinyvec::{ArrayVec, array_vec};

pub fn main() {
    let left = array_vec!([usize; SIZE] => 5, 3 , 1);

    let center = ArrayVec::<[usize; SIZE]>::new();

    let right = ArrayVec::<[usize; SIZE]>::new();

    print_toh(&left, &center, &right);
}

fn print_toh(
    left: &ArrayVec<[usize; SIZE]>,
    center: &ArrayVec<[usize; SIZE]>,
    right: &ArrayVec<[usize; SIZE]>,
) {
    let symbol = |x: Option<&usize>| {
        if let Some(v) = x {
            return v.to_string();
        } else {
            return "|".to_string();
        }
    };
    for i in (0..SIZE).rev() {
        println!(
            " {} {} {} ",
            symbol(left.get(i)),
            symbol(center.get(i)),
            symbol(right.get(i))
        )
    }
}

fn move_tower(
    left: ArrayVec<[usize; SIZE]>,
    center: ArrayVec<[usize; SIZE]>,
    right: ArrayVec<[usize; SIZE]>,
) {
    let l = left.last().map_or_else(|| -1i32, |&top| top as i32);
    let c = center.last().map_or_else(|| -1i32, |&top| top as i32);
    let r = right.last().map_or_else(|| -1i32, |&top| top as i32);
}
