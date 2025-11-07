use std::{time::Instant, path::Path};

mod calc;
mod input;
use crate::input::*;
use crate::calc::*;

fn main() {
    use input::Command as C;
    let num = read_piglins();

    // start timing after user input
    let timer = Instant::now();

    let l = calculate_dist(num);

    let elapsed = timer.elapsed();
    println!("Time: {:.2?}", elapsed);

    print_dists(&l, &vec![0.5, 0.9, 0.99]);

    print_help();
    let mut command = parse_input();

    loop {
        match command {
            C::Print(ref p) => print_dists(&l, p),
            C::Write(ref s) => write_dists(Path::new(&s)),
            C::Error => println!("Bad input!"),
            C::Help => print_help(),
            C::Exit => return
        }
        command = parse_input();
    }
}

// TODO
fn write_dists(path: &Path) {
    println!("This feature is not yet implemented :(");
}