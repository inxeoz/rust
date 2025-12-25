
pub fn main() {

    let arr : [i32; 5] = [5, 4, 1, 2, 3];

    let mut min = arr.get(0).expect("error getting max");

    for i in 1..arr.len() {

        let value = arr.get(i).expect("error getting value");

        if value <  min {min = value;}
    }

    println!("min { }", min)

}