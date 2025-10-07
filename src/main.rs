use std::io;

pub mod dist;
use crate::dist::{Dist, ConstDist};

const GR_BS: ConstDist = ConstDist::from_bounds(16, 8, 40.0);
const SS_NB: ConstDist = ConstDist::from_bounds(8, 2, 40.0);
const QUARTZ: ConstDist = ConstDist::from_bounds(12, 5, 20.0);
const OBS: ConstDist = ConstDist::from_bounds(1, 1, 40.0);
const CRY: ConstDist = ConstDist::from_bounds(3, 1, 40.0);

fn main() {

    let mut input = String::new();
    let num: usize;
    println!("Input number of piglins");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    num = input.trim().parse().expect("Bad input");

    let v = calculate_dist(num.try_into().unwrap());
    for d in v {
        d.print(vec![0.5, 0.9, 0.95, 0.99, 1.0]);
    }
}

fn calculate_dist(n: u16) -> Vec<Dist> {
    let gr_bs = Dist::from("Gravel & Blackstone", &GR_BS);
    let ss_nb = Dist::from("Soul Sand & Nether Brick", &SS_NB);
    let cry = Dist::from("Crying Obsidian", &CRY);
    let obs = Dist::from("Obsidian", &OBS);
    let quartz = Dist::from("Nether Quartz", &QUARTZ);

    let mut list = [gr_bs, ss_nb, cry, obs, quartz];

    if n == 1 { return Vec::from(list); }

    // skip the first 1, because we already have one of the pdf by default
    let start = 14 - n.leading_zeros();

    for dist in list.iter_mut() {
        // iterate over the bits of n
        for i in 0..start + 1 {
            dist.double();
            if (n >> (start - i)) & 1 != 0 {
                dist.add_original();
            }
        }
    }
    Vec::from(list)
}
