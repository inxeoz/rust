
use std::collections::{hash_map, HashMap};
pub fn main() {

    const ARR : [i32; 10] = [
        1, 2, 4, 2, 3, 0, 1, 2, 4, 0
    ];

   let mut freq : HashMap<i32, i32> = HashMap::<i32, i32>::new();


    for i in 0..ARR.len() {

        if ! freq.contains_key(&ARR[i]) {

            let mut count  =1;

            for j in i+1..ARR.len() {

                if ARR[j] == ARR[i] {
                    count += 1;
                }
            }

            freq.insert(ARR[i], count);
        }
    }

    println!("arr {ARR:?}");

    println!(" DICT {freq:?}");

}