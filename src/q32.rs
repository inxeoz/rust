pub fn main() {
    const LIST: [i32; 10] = [1, 4, 6, 12, 45, 56, 57, 89, 90, 90];

    let finding_value = 8;

    let mut low = 0;
    let mut high = LIST.len() - 1;

    while low < high {
        let mid = ((low + high) as f32 / 2f32).ceil() as usize;

        let value = LIST[mid];

        if value == finding_value {
            println!("index {mid}");

            return;
        } else if value < finding_value {
            low = mid;
        } else if value > finding_value {
            high = mid;
        } else {
            break;
        }
    }
    println!("low {low} and high {high}");
}
