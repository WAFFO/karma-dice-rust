extern crate rand;
extern crate regex;

use std::env;
use std::process;
use regex::Regex;
use rand::Rng;

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
        process::exit(0);
    }

    let x = dice_pattern.captures(&args[1]).unwrap();

    // this is the most Rusty thing I have ever seen thus far.
    let rolls: u32 = if let Ok(s) = x[1].to_string().parse() { s } else { 1 };
    let faces: u32 = if let Ok(s) = x[2].to_string().parse() { s } else { 0 } ;
    let addition: i32 = if let Ok(s) = x[3].to_string().parse() { s } else { 0 };

    if faces <= 0 {
        println!("Dice need to have at least 2 sides");
        process::exit(0);
    }

    println!("number of rolls: {}\nfaces: {}\naddition: {}", rolls, faces, addition);

    let x: String = handle_roll(faces, rolls, addition);

    println!("\n{}", x);

}

fn handle_roll(faces: u32, number_of_times: u32, addition: i32) -> String {
    let mut result: String = "[ ".to_string(); // equivilant to .to_owned(), but is more clear
    let mut sum: i32 = addition;

    for _i in 0..number_of_times {
        let temp: u32 = roll_with_karma(faces, 0.0);
        sum += temp as i32;
        result = format!("{}{}{}", result, temp, if _i+1 < number_of_times {", "} else {" ]"});
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

fn roll_with_karma(size: u32, karma: f64) -> u32 {
    // make distribution array
    let d: Vec<f64> = create_distro_array(size, karma, 0.5);
    // get a float, ranges from inclusive 0 to exclusive 1.
    let r: f64 = rand::thread_rng().gen();
    // find our face
    let mut i: usize = 0;
    while r > d[i] {
        i += 1;
    };
    // TODO: affect karma
    return (i + 1) as u32;
}

fn create_distro_array(size: u32, karma: f64, period: f32) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::new();

    // TODO: make this a real karma influenced distro array (just testing other functions for now)
    for i in 0..size {
        v.push((i as f64 + 1.0) / (size as f64));
    }

    return v;
}