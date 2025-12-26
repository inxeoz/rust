


pub fn main () {


    const ARR: [i32; 5] = [1, 2, 2, 4, 6];
    const ARR2: [i32; 7] = [4, 5, 5, 8, 9, 10, 10];


    println!("arr {ARR:?}");

    println!("arr2 {ARR2:?}");

    const ARR_LEN  : usize = ARR.len() + ARR2.len();


    let  mut new_arr = [0; ARR_LEN];


    let mut arr_i = 0;
    let mut arr2_i = 0;

    let mut new_arr_i = 0;


    while  new_arr_i <= ARR_LEN - 1 {


        if arr_i > ARR.len() -1 {

            new_arr[new_arr_i] = ARR2[arr2_i];
            arr2_i += 1;

            new_arr_i +=1;
            continue;
        }


        if arr2_i > ARR2.len() - 1{

            new_arr[new_arr_i] = ARR[arr_i];
            arr_i += 1;
            new_arr_i +=1;
            continue;
        }





        let arr_v = ARR[arr_i];
        let arr2_v = ARR2[arr2_i];

        if arr_v < arr2_v {

            new_arr[new_arr_i] = arr_v;
            arr_i +=1;

        }else {

            new_arr[new_arr_i] = arr2_v;
            arr2_i +=1;

        }




        new_arr_i += 1;


    }


    println!("combined \n {new_arr:?}");

}