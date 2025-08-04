use std::io;

pub mod dist;
use crate::dist::Dist;

fn main() {

    let mut input = String::new();
    let mut num: usize;
    let mut v: Vec<Dist>;

    loop {
        println!("Input number of piglins");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        num = input.trim().parse().expect("Bad input");

        v = calculate_dist(num);
        for d in v {
            println!("{}:", d.name);
            println!("    50th percentile: {}", d.percentile(0.5));
            println!("    90th percentile: {}", d.percentile(0.9));
            println!("    99th percentile: {}", d.percentile(0.99));
            println!("    100th percentile: {}", d.max);
            //println!("    PDF: {:?}", d.pdf);
        }
    }
}

fn calculate_dist(n: usize) -> Vec<Dist> {
    let gravel_c = Dist::from_bounds("Gravel", 16, 8, 40.0);
    let blackstone_c = Dist::from_bounds("Blackstone", 16, 8, 40.0);
    let soul_sand_c = Dist::from_bounds("Soul Sand", 8, 2, 40.0);
    let crying_obsidian_c = Dist::from_bounds("Crying Obsidian", 3, 1, 40.0);
    let obsidian_c = Dist::from_bounds("Obsidian", 1, 1, 40.0);
    let nether_brick_c = Dist::from_bounds("Nether Brick", 8, 2, 40.0);
    let nether_quartz_c = Dist::from_bounds("Nether Quartz", 12, 5, 20.0);

    let gravel = gravel_c.clone();
    let blackstone = blackstone_c.clone();
    let soul_sand = soul_sand_c.clone();
    let crying_obsidian = crying_obsidian_c.clone();
    let obsidian = obsidian_c.clone();
    let nether_brick = nether_brick_c.clone();
    let nether_quartz = nether_quartz_c.clone();

    let v_const = vec![gravel_c, blackstone_c, soul_sand_c, crying_obsidian_c, obsidian_c, nether_brick_c, nether_quartz_c];
    let mut v_mut = vec![gravel, blackstone, soul_sand, crying_obsidian, obsidian, nether_brick, nether_quartz];
    
    for (index, d) in v_const.iter().enumerate() {
        let mut count: usize = 1;
        while 2 * count <= n {
            v_mut[index].add_self();
            count *= 2;
        }
        while count < n {
            v_mut[index].add(d);
            count += 1;
        }
    }
    v_mut
}
