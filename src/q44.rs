// use std::thread;
// use std::time::Duration;
// use tinyvec::ArrayVec;
// const MAX_DISK: usize = 3;
// const MAX_POLS: usize = 3;
// type Tower = ArrayVec<[u32; MAX_DISK]>;

// struct Towers {
//     poles: [Tower; MAX_POLS],
// }

// fn print_toh(t: &Towers) {
//     println!();
//     thread::sleep(Duration::from_secs(1));

//     let symbol = |x: Option<&usize>| x.map(|v| v.to_string()).unwrap_or("|".to_string());

//     for i in (0..MAX_DISK).rev() {
//         println!(
//             " {} {} {} ",
//             symbol(t.poles[0].get(i)),
//             symbol(t.poles[1].get(i)),
//             symbol(t.poles[2].get(i))
//         )
//     }
// }

// fn move_tower(n: usize, from: usize, to:usize, aux:usize, towers: &mut Towers) {

//     if n==0 {
//         return;
//     }

//     move_tower(n-1, from, aux, to, towers);

//     let disk = towers.poles[from].pop().unwrap();

//     towers.poles[to].push(disk);

//     print_toh(towers);

//     move_tower(n - 1, aux, to, from, towers);

// }
