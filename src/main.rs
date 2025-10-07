use std::{io, time::Instant};

mod calc;
use crate::calc::*;

fn main() {
    let num = read_input();

    // start timing after user input
    let timer = Instant::now();

    let l = calculate_dist(num);

    print_dists(l);

    let elapsed = timer.elapsed();
    println!("Time: {:.2?}", elapsed);
}

// Get number of piglins from user
fn read_input() -> u16 {
    let mut input = String::new();
    println!("Input number of piglins");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Bad input")
}