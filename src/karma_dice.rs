use rand::Rng;

pub fn hello<'a>() -> &'a  str {
    "Hello from Karma_Dice module!"
}

/// returns (rolls: Vec<u32>, sum: i32, karma: f64)
pub fn handle_roll(faces: u32, number_of_rolls: u32, addition: i32, karma: f64) -> (Vec<u32>, i32, f64) {
    let mut rolls: Vec<u32> = Vec::new();
    let mut sum: i32 = addition;
    let mut karma: f64 = karma;

    for _i in 0..number_of_rolls {
        let temp: u32 = (fast_karma_rng(&mut karma)*(faces as f64)).ceil() as u32;
        sum += temp as i32;
        println!("temp = {}", temp);
        rolls.push(temp);
    }
    return (rolls, sum, karma);
}

/// returns JSON String
pub fn handle_roll_string(faces: u32, number_of_rolls: u32, addition: i32, karma: f64) -> String {
    // returns (rolls: Vec<u32>, sum: i32, karma: f64)
    let result = handle_roll(faces, number_of_rolls, addition, karma);

    // equivalent to .to_owned(), but is more clear
    let mut rolls: String = "[ ".to_string();

    for _i in 0..result.0.len() {
        rolls = format!("{}{}{}", rolls, result.0[_i], if _i+1 < number_of_rolls as usize {", "} else {" ]"});
    }
    // if you intended to print `}`, you can escape it using `}}`
    return format!("{{ \"rolls\": {}, \"addition\": {}, \"sum\": {}, \"karma\": {} }}", rolls, addition, result.1, result.2);
}

/// mutates karma, returns a random f64 from [0, 1)
pub fn karma_rng(karma: &mut f64) -> f64 {
    let influence = *karma / (1.0 + karma.abs());
    let r: f64 = rand::thread_rng().gen();

    use std::f64::consts::PI;
    let result: f64 = r + if *karma >= 0.0 {
        (1.0+(r*PI).cos())/10.0*influence
    } else {
        (1.0-(r*PI).cos())/10.0*influence
    };

    *karma += affect_karma(r);

    println!("r = {}\nresult = {}",r,result);

    result
}

/// takes in a float from 0 to 1
fn affect_karma(roll: f64) -> f64 {
    (roll as f64) * -2.0 + 1.0
}

/// mutates karma, returns a random f64 from [0, 1), uses an approximate formula for cos
pub fn fast_karma_rng(karma: &mut f64) -> f64 {
    let influence = *karma / (1.0 + karma.abs());
    let r: f64 = rand::thread_rng().gen();

    use std::f64::consts::PI;
    let result: f64 = r + if *karma >= 0.0 {
        (1.0+fast_sin(r*PI-PI/2.0))/10.0*influence
    } else {
        (1.0-fast_sin(r*PI-PI/2.0))/10.0*influence
    };

    *karma += affect_karma(r);

    println!("r = {}\nresult = {}",r,result);

    result
}

fn fast_sin(x: f64) -> f64 {
    use std::f64::consts::PI;
    (16.0 * x * (PI - x)) / (5.0 * PI.powi(2) - 4.0 * x * (PI - x))
}