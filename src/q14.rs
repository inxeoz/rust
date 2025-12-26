


pub fn main() {
    const ARR: [i32; 5] = [1, 2, 3, 4, 5];

    const SHIFT_BY : usize = 2;

    let mut new_arr = [0; ARR.len()];

    println!("arr {ARR:?}");


    for i in 0..ARR.len() {

        let new_i  =  ( i + SHIFT_BY  ) % ARR.len();


        new_arr[new_i] = ARR[i];
    }

    println!("shift by {SHIFT_BY} {:?}", new_arr)
}
