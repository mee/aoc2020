use clap::{Arg, App};

#[macro_use]
extern crate lazy_static;

fn main() {
    let matches = App::new("Advent of Code 2020")
        .version("1.0.0")
        .author("Mike Erickson <mike.erickson@gmail.com>")
        .arg(Arg::with_name("day")
             .short("d")
             .long("day")
             .takes_value(true)
             .help("day or exercise"))
        .get_matches();

        match matches.value_of("day").expect("day not specfied") {
            "1" => day1(),
            "2" => day2(),
            "3" => day3(),
            "4" => day4(),
            _ => println!("Invalid day specified"),
        }
}

// day 1
fn day1() {

    let reader = include_str!("1.input");

	let mut nums = Vec::<usize>::new();
	for line in reader.lines() {
		nums.push(line.parse().unwrap());
	}

	nums.sort();

	if let Some(product) = test2(&nums, 2020, 0, nums.len()-1) {
		println!("product of two numbers that sum to 2020: {}", product);
	}

    for j in 1..nums.len()-2 {
        let value = nums.remove(j);
        if let Some(product) = test2(&nums, 2020 - value, 0, nums.len()-1) {
            println!("product of three numbers that sum to 2020 {}", product * value);
            break;
        } else {
            nums.insert(j, value);
        }
    }
}

fn test2(list : &[usize], desired_sum: usize, i : usize, j : usize) -> Option<usize> {
	if i == j {
		return None;
	} else {
		let sum = list[i] + list[j];
		if sum == desired_sum {
			return Some(list[i] * list[j]);
		} else if sum > desired_sum {
			return test2(list, desired_sum, i, j-1);
		} else {
			return test2(list, desired_sum, i+1, j);
		}
	}
}

// day 2
fn day2() {
    let input = include_str!("2.input");

    // "1-2 a: asdf"
    let mut valid = 0;
    let mut valid2 = 0;
    let mut total = 0;
    for line in input.lines() {
        let tokens : Vec<&str> = line.split(' ').collect();
        let range : Vec<&str> = tokens[0].split("-").collect();
        let letter = tokens[1].chars().nth(0).unwrap();
        let password = tokens[2];

        let low: usize = range[0].parse().unwrap();
        let high: usize = range[1].parse().unwrap();

        let count = password.matches(letter).collect::<Vec<&str>>().len();
        if count >= low && count <= high {
            valid = valid + 1;
        }

        let low_char = password.chars().nth(low-1).unwrap();
        let high_char = password.chars().nth(high-1).unwrap();

        if low_char == letter {
            if high_char != letter {
                valid2 = valid2 + 1;
            }
        } else {
            if high_char == letter {
                valid2 = valid2 + 1;
            }
        }
        total = total + 1;
    }

    println!("{} of {} passwords are valid according to policy 1", valid, total);
    println!("{} of {} passwords are valid according to policy 2", valid2, total);
}

// day 3
fn day3() {
    let input = include_str!("3.input");

    #[derive(Copy, Clone, Debug)]
    struct Cursor { pos: usize, right: usize, down: usize, trees: usize };
    impl Cursor {
        fn new(r: usize, d: usize) -> Cursor {
            return Cursor { pos: 0, right: r, down: d, trees: 0 };
        }
        fn shift(&mut self, width: usize) {
            self.pos = (self.pos + self.right) % (width + 1);
        }
        fn tree(&mut self) {
            self.trees = self.trees + 1;
        }
    }
    let mut cursors = vec![
        Cursor::new(1, 1),
        Cursor::new(3, 1),
        Cursor::new(5, 1),
        Cursor::new(7, 1),
        Cursor::new(1, 2),
    ];

    let mut width = 0;
    for (index, line) in input.lines().enumerate() {
        if index == 0 {
            width = line.len() - 1;
            continue
        }
        for cursor in &mut cursors {
            if (index % cursor.down) == 0 {
                cursor.shift(width);
                if line.chars().nth(cursor.pos).unwrap() == '#' {
                    cursor.tree();
                }
            }
        }
    }
    let mut trees = 1;
    for cursor in cursors {
        trees = trees * cursor.trees;
    }
    println!("Found {} trees", trees);
}


// day 4
fn day4() {
    use std::collections::HashMap;
    use regex::Regex;

    let input = include_str!("4.input");

    /*
    // don't care about values, but probably will later
    struct Passport { 
        byr: &str, // (Birth Year)
        iyr: &str, // (Issue Year)
        eyr: &str, // (Expiration Year)
        hgt: &str, // (Height)
        hcl: &str, // (Hair Color)
        ecl: &str, // (Eye Color)
        pid: &str, // (Passport ID)
        cid: &str, // (Country ID)
    }
    */

    fn is_valid(kvs: &HashMap<&str, &str>) -> bool {
        lazy_static! { static ref REQUIRED_KEYS: Vec<&'static str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*"cid"*/]; }
        for required_key in REQUIRED_KEYS.iter() {
            if !kvs.contains_key(required_key) {
                return false;
            }
        }

        if !kvs["byr"].parse::<usize>().map(|v| v >= 1920 && v <= 2002).unwrap() {
            return false;
        }
        if !kvs["iyr"].parse::<usize>().map(|v| v >= 2010 && v <= 2020).unwrap() {
            return false;
        }
        if !kvs["eyr"].parse::<usize>().map(|v| v >= 2020 && v <= 2030).unwrap() {
            return false;
        }

        lazy_static! { static ref RE_HGT: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap(); }
        if !match RE_HGT.captures(&kvs["hgt"]) {
            Some(caps) => {
                match caps.get(1).unwrap().as_str().parse::<usize>() {
                    Ok(hgt) => {
                        match caps.get(2) {
                            Some(unit) if unit.as_str() == "cm" => hgt >= 150 && hgt <= 193,
                            Some(unit) if unit.as_str() == "in" => hgt >= 59 && hgt <= 76,
                            _ => false
                        }
                    },
                    Err(_) => false,
                }
            },
            None => false,
        } { return false; }

        lazy_static! { static ref RE_HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap(); }
        if !RE_HCL.is_match(&kvs["hcl"]) {
            return false;
        }

        lazy_static! { static ref VALID_ECL: Vec<&'static str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]; }
        if !VALID_ECL.contains(&kvs["ecl"]) {
            return false;
        }

        lazy_static! { static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap(); }
        if !RE_PID.is_match(&kvs["pid"]) {
            return false;
        }

        return true;
    }

    let mut valid = 0;
    let mut kvs: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        if line == "" {
            if is_valid(&kvs) {
                valid = valid + 1;
            }
            kvs.clear();
        }
        for pair in line.split(' ') {
            let row_pairs = pair.split(':').collect::<Vec<&str>>();
            if let [key, value] = &row_pairs[..] {
                kvs.insert(key, value);
            }
        }
    }
    if is_valid(&kvs) {
        valid = valid + 1;
    }
    println!("Found {} valid passports", valid);
}
