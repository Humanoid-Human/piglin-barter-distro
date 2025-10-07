use std::{io, time::Instant};

mod calc;
use crate::calc::*;

fn main() {

    let mut input = String::new();
    println!("Input number of piglins");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: usize = input.trim().parse().expect("Bad input");

    let timer = Instant::now();

    let l = calculate_dist(num.try_into().unwrap());

    print_dists(l);

    let elapsed = timer.elapsed();
    println!("Time: {:.2?}", elapsed);
}
