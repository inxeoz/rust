


pub fn main() {


    let mut arr : [ i32; 5] = [1, 1, 2, 3, 4];

    let mut slow = 0;


    //if both value are same skip that

    for i in 1..arr.len() {

        if arr[slow] == arr[i] {
            continue
        }else{
            slow +=1; // new place to save
            arr[slow] = arr[i];
        }
    }
    for i in  0..slow + 1 {
        print!("{} ", arr[i]);
    }

}