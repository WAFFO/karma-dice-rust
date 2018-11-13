extern crate regex;

use std::env;
use std::process;
use regex::Regex;

// my user defined mod
extern crate karma_dice_rust;
use karma_dice_rust::karma_dice;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dice_pattern = Regex::new(r"([1-9]?[0-9]*)d([1-9]?[0-9]*)((?:[+,\-][1-9][0-9]*)?)").unwrap();

    if args.len() < 2 || !dice_pattern.is_match(&args[1]) {
        println!("Please write dice roll as: [number of rolls]d<faces>[+|-constant] [karma] \nFor example:\n1d20+3 0.5\nd100\n2d20 -1.35");
        process::exit(0);
    }

    let x = dice_pattern.captures(&args[1]).unwrap();

    // this is the most Rusty thing I have ever seen thus far.
    let rolls: u32 = if let Ok(s) = x[1].to_string().parse() { s } else { 1 };
    let faces: u32 = if let Ok(s) = x[2].to_string().parse() { s } else { 0 } ;
    let addition: i32 = if let Ok(s) = x[3].to_string().parse() { s } else { 0 };
    let karma: f64 = if args.len() > 2 { if let Ok(s) = args[2].to_string().parse() { s } else { 0.0 } } else { 0.0 };

    if faces <= 0 {
        println!("Dice need to have at least 2 sides");
        process::exit(0);
    }

    // println!("number of rolls: {}\nfaces: {}\naddition: {}\nkarma: {}", rolls, faces, addition, karma);

    let x: String = karma_dice::handle_roll(faces, rolls, addition, karma);

    println!("\n{}", x);
}