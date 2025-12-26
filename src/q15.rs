


pub fn main() {

    // const ARR : [ i32; 5] = [4, 2, 1, 4, 5];

    const ARR: [i32; 5] = [0, 1, 1, 2, 3];


    let mut min = ARR[0];

    for i in 1..ARR.len() {

        if min > ARR[i]  {
           println!("NOT SORTED IN ASC");
            return;
        }else{
            min  = ARR[i];
        }


    }

    println!("SORTED ASC");




}