


pub fn main() {

    const ARR: [i32; 8] = [4, 2, 4, 90, 23, 0, 0, 2];


    let mut lrg;
    let mut slrg ;

    if ARR[0] > ARR[1] {
        lrg = ARR[0];
        slrg = ARR[1];
    }else{

        lrg = ARR[1];
        slrg = ARR[0];

    }


    for i in 2..ARR.len() {

        let new_v = ARR[i];

        if new_v > lrg {
            slrg = lrg;
            lrg = new_v;
            continue;
        }


        if new_v > slrg {

            slrg = new_v;
        }



    }

    println!("arr {ARR:?}");
    println!("largest {lrg}");
    println!("second largest {slrg}");



}