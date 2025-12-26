

pub fn main() {


    let mut arr = [1, 0, 3, 4, 0, 2, 3, 0, 0];


    println!("before {arr:?}");


    let mut last_i = arr.len() -1 ;


    for i in 0..arr.len() {

        while arr[last_i] == 0 && last_i > 0 {
            last_i -=1;
        }


        if i >= last_i {
            break;
        }




        if arr[i] == 0 {
            let temp = arr[last_i];
            arr[last_i] = 0;
            arr[i] = temp;
        }





    }

    println!("after {arr:?}");

}