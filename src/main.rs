use std::{io::{Write, BufWriter}, fs, path::Path, time::Instant};

mod calc;
mod input;
use input::*;
use calc::*;

const DIST_NUM: usize = 5;
const DENOM: f64 = 459.0;

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
            C::Write(ref s) => write_dists(Path::new(&s), &l),
            C::Error => println!("Bad input!"),
            C::Help => print_help(),
            C::Exit => return
        }
        command = parse_input();
    }
}

fn write_dists(path: &Path, dists: &[Dist; DIST_NUM]) {
    let file;
    match fs::File::create(path) {
        Ok(f) => file = f,
        Err(_) => {
            println!("Error creating/opening file.");
            return;
        }
    }

    let mut w = BufWriter::new(file);

    // header
    if w.write_all(b"name,num,prob\n").is_err() {
        println!("Failed to write to file.\n");
        return;
    }

    // data
    for dist in dists {
        if w.write_all(dist.to_csv().as_bytes()).is_err() {
            println!("Failed to write to file.\n");
            return;
        }
    }

    // flush buffer
    if w.flush().is_err() {
        println!("Failed to write to file.\n");
        return;
    }

    println!("Write complete.");
}

fn print_dists(l: &[Dist; DIST_NUM], percentiles: &Vec<f64>) {
    for p in percentiles {
        if *p > 1.0 || *p < 0.0 {
            println!("Bad percentile given: {:.2}", p);
            return;
        }
    }
    for d in l {
        d.print(percentiles);
    }
}