use std::thread;
use std::time::Duration;
use tinyvec::ArrayVec;
const MAX_DISK: usize = 3;
const MAX_POLS: usize = 3;
type Tower = ArrayVec<[u32; MAX_DISK]>;

struct Towers {
    poles: [Tower; MAX_POLS],
}

fn print_toh(t: &Towers) {
    println!();
    thread::sleep(Duration::from_secs(1));

    let symbol = |x: Option<&usize>| x.map(|v| v.to_string()).unwrap_or("|".to_string());

    for i in (0..MAX_DISK).rev() {
        println!(
            " {} {} {} ",
            symbol(t.poles[0].get(i)),
            symbol(t.poles[1].get(i)),
            symbol(t.poles[2].get(i))
        )
    }
}

fn main() {}
