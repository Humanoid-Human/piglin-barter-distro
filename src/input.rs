use std::io;

// Get number of piglins from user
pub fn read_piglins() -> u16 {
    println!("Input number of piglins");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Bad input")
}

pub enum Command {
    Print(Vec<f64>), Write(String), Exit, Error, Help
}

pub fn print_help() {
    println!("Options:");
    println!("  p [list]    Print percentiles. [list] is comma-separated.");
    println!("  w [path]    Write the distributions to a csv file at [path].");
    println!("  h           Show this message");
    println!("  e           Close the program");
}

pub fn parse_input() -> Command {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");

    if input.len() < 1 { return Command::Error; }

    match input.chars().nth(0).unwrap() {
        'p' => Command::Print(percentiles(&input[2..])),
        'w' => Command::Write(String::from(&input[2..])),
        'e' => Command::Exit,
        'h' => Command::Help,
        _ => Command::Error
    }
}

fn percentiles(input: &str) -> Vec<f64> {
    input.split(",")
        .map(|x| 
            x.trim().parse::<f64>().expect("Bad input") / 100.0)
        .collect()
}