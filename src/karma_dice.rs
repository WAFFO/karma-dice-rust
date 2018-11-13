use rand::Rng;

pub fn hello() -> String {
    return "Hello from Karma_Dice module!".to_string();
}

pub fn handle_roll(faces: u32, number_of_rolls: u32, addition: i32, karma: f64) -> String {
    let mut result: String = "{ \"rolls\": [ ".to_string(); // equivalent to .to_owned(), but is more clear
    let mut sum: i32 = addition;
    let mut karma: f64 = karma;

    for _i in 0..number_of_rolls {
        let temp: u32 = roll_with_karma(faces, karma);
        karma += affect_karma(faces, temp);
        sum += temp as i32;
        result = format!("{}{}{}", result, temp, if _i+1 < number_of_rolls {", "} else {" ]"});
    }
    // if you intended to print `}`, you can escape it using `}}`
    return format!("{}, \"addition\": {}, \"sum\": {}, \"karma\": {} }}", result, addition, sum, karma);
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

fn create_distro_array(size: u32, karma: f64, period: f64) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::new();
    let sizef : f64 = size as f64;
    let influence: f64 = ((karma/2.0)/(1.0+(karma/2.0).abs()))*(0.01+(1.45/(1.0+(sizef/2.4).powf(1.3))));

    use std::f64::consts::PI;
    for _i in 0..size {
        v.push((((1.0+period)*PI/(sizef-1.0))*((_i as f64)+1.0)-0.5*PI*period).cos()/2.0*influence);
    }

    let mut i : usize = (size-2) as usize;
    loop {
        v[i] += v[i+1];
        if i > 0 { i -= 1; } else { break; }
    }

    let big_shift: f64 = v[size as usize -1];
    for _i in 0..size {
        v[_i as usize] += ((1.0/sizef)*(_i as f64 +1.0)) as f64 -big_shift;
    }
//
    return v;
}

fn affect_karma(max: u32, roll: u32) -> f64 {
    return (roll as f64 / (max - 1) as f64)*-2.0 + 1.0;
}