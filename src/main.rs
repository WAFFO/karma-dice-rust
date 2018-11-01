extern crate rand;
extern crate regex;

use std::env;
use std::process;
use regex::Regex;

fn main() {
    let mut x: u8 = 0;
    for argument in env::args() {
        println!("Arg [{}]: {}", x, argument);
        x += 1;
    }

    let args: Vec<String> = env::args().collect();

    let dice_pattern = Regex::new(r"([1-9]?[0-9]*)d([1-9]?[0-9]*)((?:[\+,\-][1-9][0-9]*)?)").unwrap();

    if args.len() < 2 || !dice_pattern.is_match(&args[1]) {
        println!("Please write dice roll as: [number of rolls]d[dice size][+|-addition]\nFor example: 1d20+3");
        process::exit(1);
    }


    let x = dice_pattern.captures(&args[1]).unwrap();

    let rolls: u32 = if x[1].to_string() == "" { 1 } else { x[1].to_string().parse().unwrap() };
    let faces: u32 = if x[2].to_string() == "" { 0 } else { x[2].to_string().parse().unwrap() };
    let addition: i32 = if x[3].to_string() == "" { 0 } else { x[3].to_string().parse().unwrap() };

    if faces <= 0 {
        println!("Dice need to have at least 2 sides");
        process::exit(1);
    }

    println!("number of rolls: {}\nfaces: {}\naddition: {}", rolls, faces, addition);

    let x: String = handleRoll(faces, rolls, addition);

    println!("{}", x);

}

fn handleRoll(faces: u32, number_of_times: u32, addition: i32) -> String {
    let mut result: String = "[ ".to_string(); // equivilant to .to_owned(), but is more clear
    let mut sum: i32 = addition;

    for i in 0..number_of_times {
        let temp: u32 = rollWithKarma(faces, 0.0);
        sum += temp as i32;
        result = format!("{}{}{}", result, temp, if i+1 < number_of_times {", "} else {" ]"});
    }
    if addition > 0 {
        return format!("{} + {} = {}", result, addition, sum);
    }
    else if addition < 0 {
        return format!("{} - {} = {}", result, addition*-1, sum);
    }
    else {
        return format!("{} = {}", result, sum);
    }
}

fn rollWithKarma(size: u32, karma: f64) -> u32 {
    return 10;
}