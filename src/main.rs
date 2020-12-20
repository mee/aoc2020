use clap::{App, Arg};
use num_traits::pow;

#[macro_use]
extern crate lazy_static;

fn main() {
    let matches = App::new("Advent of Code 2020")
        .version("1.0.0")
        .author("Mike Erickson <mike.erickson@gmail.com>")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("day or exercise"),
        )
        .get_matches();

    match matches.value_of("day").expect("day not specfied") {
        "1" => day1(),
        "2" => day2(),
        "3" => day3(),
        "4" => day4(),
        "5" => day5(),
        "6" => day6(),
        "7" => day7(),
        "8" => day8::day8(),
        "9" => day9::day9(),
        "10" => day10::day10(),
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

    if let Some(product) = test2(&nums, 2020, 0, nums.len() - 1) {
        println!("product of two numbers that sum to 2020: {}", product);
    }

    for j in 1..nums.len() - 2 {
        let value = nums.remove(j);
        if let Some(product) = test2(&nums, 2020 - value, 0, nums.len() - 1) {
            println!(
                "product of three numbers that sum to 2020 {}",
                product * value
            );
            break;
        } else {
            nums.insert(j, value);
        }
    }
}

fn test2(list: &[usize], desired_sum: usize, i: usize, j: usize) -> Option<usize> {
    if i == j {
        return None;
    } else {
        let sum = list[i] + list[j];
        if sum == desired_sum {
            return Some(list[i] * list[j]);
        } else if sum > desired_sum {
            return test2(list, desired_sum, i, j - 1);
        } else {
            return test2(list, desired_sum, i + 1, j);
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
        let tokens: Vec<&str> = line.split(' ').collect();
        let range: Vec<&str> = tokens[0].split("-").collect();
        let letter = tokens[1].chars().nth(0).unwrap();
        let password = tokens[2];

        let low: usize = range[0].parse().unwrap();
        let high: usize = range[1].parse().unwrap();

        let count = password.matches(letter).collect::<Vec<&str>>().len();
        if count >= low && count <= high {
            valid = valid + 1;
        }

        let low_char = password.chars().nth(low - 1).unwrap();
        let high_char = password.chars().nth(high - 1).unwrap();

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

    println!(
        "{} of {} passwords are valid according to policy 1",
        valid, total
    );
    println!(
        "{} of {} passwords are valid according to policy 2",
        valid2, total
    );
}

// day 3
fn day3() {
    let input = include_str!("3.input");

    #[derive(Copy, Clone, Debug)]
    struct Cursor {
        pos: usize,
        right: usize,
        down: usize,
        trees: usize,
    };
    impl Cursor {
        fn new(r: usize, d: usize) -> Cursor {
            return Cursor {
                pos: 0,
                right: r,
                down: d,
                trees: 0,
            };
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
            continue;
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
use std::collections::HashMap;
fn day4() {
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

        if !kvs["byr"]
            .parse::<usize>()
            .map(|v| v >= 1920 && v <= 2002)
            .unwrap()
        {
            return false;
        }
        if !kvs["iyr"]
            .parse::<usize>()
            .map(|v| v >= 2010 && v <= 2020)
            .unwrap()
        {
            return false;
        }
        if !kvs["eyr"]
            .parse::<usize>()
            .map(|v| v >= 2020 && v <= 2030)
            .unwrap()
        {
            return false;
        }

        lazy_static! {
            static ref RE_HGT: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
        }
        if !match RE_HGT.captures(&kvs["hgt"]) {
            Some(caps) => match caps.get(1).unwrap().as_str().parse::<usize>() {
                Ok(hgt) => match caps.get(2) {
                    Some(unit) if unit.as_str() == "cm" => hgt >= 150 && hgt <= 193,
                    Some(unit) if unit.as_str() == "in" => hgt >= 59 && hgt <= 76,
                    _ => false,
                },
                Err(_) => false,
            },
            None => false,
        } {
            return false;
        }

        lazy_static! {
            static ref RE_HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        }
        if !RE_HCL.is_match(&kvs["hcl"]) {
            return false;
        }

        lazy_static! {
            static ref VALID_ECL: Vec<&'static str> =
                vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        }
        if !VALID_ECL.contains(&kvs["ecl"]) {
            return false;
        }

        lazy_static! {
            static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
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

// day 5

fn row(pass: &str) -> usize {
    let mut row = 0;
    for (index, ch) in pass.chars().take(7).enumerate() {
        if ch == 'B' {
            row = row + pow(2, 7 - index - 1);
        }
    }
    return row;
}

fn col(pass: &str) -> usize {
    let mut col = 0;
    for (index, ch) in pass.chars().skip(7).enumerate() {
        if ch == 'R' {
            col = col + pow(2, 3 - index - 1);
        }
    }
    return col;
}

// closed form solution for sum(i..k)
fn cumsum(k: usize) -> usize {
    k * (k + 1) / 2
}

fn day5() {
    let input = include_str!("5.input");
    let mut max_id = 0;
    let mut min_id = 1000;
    let mut sum_id = 0;
    for line in input.lines() {
        let seat_id = row(line) * 8 + col(line);
        if seat_id > max_id {
            max_id = seat_id;
        }
        if seat_id < min_id {
            min_id = seat_id;
        }
        sum_id = sum_id + seat_id;
    }

    let expected_sum = cumsum(max_id) - cumsum(min_id - 1);
    let my_seat_id = expected_sum - sum_id;

    println!("The maximum seat id found in the input is {}", max_id);
    println!("My seat id is {}", my_seat_id);
}

#[test]
fn test_row() {
    assert_eq!(row("BFFFBBFRRR"), 70);
    assert_eq!(row("FFFBBBFRRR"), 14);
    assert_eq!(row("BBFFBBFRLL"), 102);
}

#[test]
fn test_col() {
    assert_eq!(col("BFFFBBFRRR"), 7);
    assert_eq!(col("FFFBBBFRRR"), 7);
    assert_eq!(col("BBFFBBFRLL"), 4);
}

#[test]
fn test_cumsum() {
    assert_eq!(cumsum(3), 6);
    assert_eq!(cumsum(4), 10);
}

use std::collections::HashSet;
// day 6
fn day6() {
    let input = include_str!("6.input");

    let mut mark = true;
    let mut sum = 0;
    let mut group_qs: HashSet<char> = HashSet::new();
    for line in input.lines() {
        if line == "" {
            sum = sum + group_qs.len();
            group_qs.clear();
            mark = true;
        } else {
            if mark {
                group_qs = line.chars().collect::<HashSet<char>>();
                mark = false;
            } else {
                let person_qs: HashSet<char> = line.chars().collect();
                group_qs = group_qs.intersection(&person_qs).cloned().collect();
            }
        }
    }
    sum = sum + group_qs.len();

    println!("The sum of common questions per group is {}", sum);
}

// day 7

// dynamic programming alarm bells going off!

use multimap::MultiMap;
//use std::collections::HashMap;

fn day7() {
    let input = include_str!("7.input");

    // store key `is contained in` value relationships
    let mut is_contained_in: MultiMap<&str, &str> = MultiMap::new();
    let mut contains: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    // "muted lime bags contain 1 wavy lime bag, 1 vibrant green bag, 3 light yellow bags."
    // "dotted teal bags contain no other bags."
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let subject = parts[0];
        let objects = parts[1];
        if objects == "no other bags." {
            continue;
        } else {
            let objects = parts[1].split(", ");
            for object in objects {
                let object_parts = object
                    .trim_end_matches(|c| c == '.' || c == ',')
                    .trim_end_matches("bags")
                    .trim_end_matches("bag")
                    .trim_end()
                    .splitn(2, ' ')
                    .collect::<Vec<&str>>();
                let object_count = object_parts[0].parse::<usize>().unwrap();
                let object_color = object_parts[1];
                is_contained_in.insert(object_color, subject);
                match contains.get_mut(subject) {
                    None => {
                        let mut inners: HashMap<&str, usize> = HashMap::new();
                        inners.insert(object_color, object_count);
                        contains.insert(subject, inners);
                    }
                    Some(inners) => match inners.get_mut(object_color) {
                        None => {
                            inners.insert(object_color, object_count);
                        }
                        Some(inner) => {
                            *inner = *inner + object_count;
                        }
                    },
                }
            }
        }
    }

    let my_color = "shiny gold";

    let mut super_colors: HashSet<&str> = HashSet::new();
    find_super_colors(my_color, &is_contained_in, &mut super_colors);
    println!(
        "A total of {} colors of bags may indirectly contain a {} bag",
        super_colors.len(),
        my_color
    );
    let inner_bag_count = find_total_bag_count(my_color, &contains) - 1; // minus my_color bag
    println!("A {} bag contains {} inner bags", my_color, inner_bag_count);
}

fn find_total_bag_count<'a>(color: &str, contains: &HashMap<&str, HashMap<&str, usize>>) -> usize {
    match contains.get(color) {
        None => 1, // just me myself, ma'am
        Some(inners) => {
            let mut icc = 1; // inner color count; starting with just me
            for (inner, count) in inners {
                if &color != inner {
                    icc = icc + count * find_total_bag_count(inner, &contains);
                }
            }
            return icc;
        }
    }
}

fn find_super_colors<'a>(
    color: &str,
    is_contained_in: &MultiMap<&str, &'a str>,
    mut super_colors: &mut HashSet<&'a str>,
) {
    match is_contained_in.get_vec(color) {
        None => (),
        Some(colors) => {
            for c in colors {
                if super_colors.insert(c) {
                    find_super_colors(c, &is_contained_in, &mut super_colors);
                }
            }
        }
    }
}

mod day8 {
    use std::collections::HashSet;
    use std::str::FromStr;

    #[derive(Debug, Clone, PartialEq)]
    enum Operation {
        Nop(isize),
        Jmp(isize),
        Acc(isize),
    }
    #[derive(Debug, Clone)]
    struct OperationError;

    impl FromStr for Operation {
        type Err = OperationError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts = s.splitn(2, ' ').collect::<Vec<&str>>();
            let value = parts[1].parse::<isize>().unwrap();
            let op = match parts[0] {
                "jmp" => Operation::Jmp(value),
                "acc" => Operation::Acc(value),
                "nop" => Operation::Nop(value),
                _ => panic!("bad operation"),
            };
            Ok(op)
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Command {
        index: usize,
        op: Operation,
    }

    impl Command {
        fn new(index: usize, op: Operation) -> Command {
            Command {
                index: index,
                op: op,
            }
        }
    }
    struct ExecutionError;

    fn execute(
        index: usize,
        cmds: &Vec<Command>,
        seen: &mut HashSet<usize>,
    ) -> Result<isize, ExecutionError> {
        if index == cmds.len() {
            Ok(0)
        } else if seen.contains(&index) {
            eprintln!("Found loop at index {}", index);
            Err(ExecutionError)
        } else {
            seen.insert(index);
            eprintln!("Executing {:?}", &cmds[index]);
            match &cmds[index].op {
                Operation::Jmp(offset)
                    if (index as isize + offset) as usize > 0
                        && ((index as isize + offset) as usize) <= cmds.len() =>
                {
                    execute((index as isize + offset) as usize, &cmds, seen)
                }
                Operation::Acc(addend) => {
                    execute(index + 1, cmds, seen).and_then(|res| Ok(res + addend))
                }
                Operation::Jmp(_) => Err(ExecutionError), // jumps off cmds
                _ => execute(index + 1, cmds, seen),
            }
        }
    }

    fn flip(index: usize, cmds: &mut Vec<Command>) {
        eprintln!("Flipping {:?}", cmds[index]);
        cmds[index].op = match cmds[index].op {
            Operation::Jmp(value) => Operation::Nop(value),
            Operation::Nop(value) => Operation::Jmp(value),
            _ => panic!("bad operation"),
        };
    }

    // day 8
    pub fn day8() {
        let input = include_str!("8.input");

        let mut cmds = input
            .lines()
            .enumerate()
            .map(|(i, s)| Command::new(i, s.parse::<Operation>().unwrap()))
            .collect::<Vec<Command>>();

        for index in 0..cmds.len() {
            match cmds[index].op {
                Operation::Nop(_) | Operation::Jmp(_) => {
                    flip(index, &mut cmds);
                    match execute(0, &cmds, &mut HashSet::new()) {
                        Ok(accum) => {
                            println!("Terminated with accumulator value {}", accum);
                            break;
                        }
                        _ => {
                            flip(index, &mut cmds);
                            continue;
                        }
                    }
                }
                _ => continue,
            }
        }
    }
}

mod day9 {

    const PREAMBLE: usize = 25;

    fn is_valid(index: usize, nums: &Vec<usize>) -> bool {
        for i in (index - 25)..index - 1 {
            for j in i..index {
                if nums[i] + nums[j] == nums[index] {
                    return true;
                }
            }
        }
        return false;
    }

    // only works on sorted lists, but preamble may not be sorted...
    fn creep_for_sum(val: usize, nums: &Vec<usize>) -> (usize, usize) {
        let mut tail = 0;
        let mut head = 1;
        let mut runsum = nums[tail] + nums[head];
        while tail < nums.len() {
            if runsum < val {
                head += 1;
                runsum += nums[head];
            } else if runsum > val {
                runsum -= nums[tail];
                tail += 1;
                if head == tail {
                    head += 1;
                    runsum += nums[head];
                }
            } else {
                break;
            }
        }

        let mut min = runsum;
        let mut max = 0;
        for num in nums[tail..head].iter() {
            if num > &max {
                max = *num;
            }
            if num < &min {
                min = *num;
            }
        }
        (min, max)
    }

    pub fn day9() {
        let input = include_str!("9.input");

        let nums: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

        for i in PREAMBLE..(nums.len() - 1) {
            if !is_valid(i, &nums) {
                println!("Found {} which is not valid", &nums[i]);
                let (min, max) = creep_for_sum(nums[i], &nums);
                println!("Sum of smallest and largest in running sum to invalid number: {}", min + max);
            }
        }
    }
}

// day 10
mod day10 {
    pub fn day10() {
        let mut adapters = include_str!("10.input").lines()
        .map(|l| l.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        adapters.sort();
        if let Some(stats) = joltage_summary_stat(0, &mut adapters) {
            println!("The product of one-jumps and three-jumps is {}", stats.0*stats.1);
        } else {
            println!("No valid combinations of adapters found");
        }
    }

    // requires `adapters` be sorted - Vec::is_sorted() experimental in stable...
    pub fn joltage_summary_stat(output: usize, adapters: &mut Vec<usize>) -> Option<(usize, usize)> {
        if adapters.len() == 1 {
            if output > adapters[0] {
                return None;
            } else {
                return match adapters[0] - output {
                    1 => Some((1, 1)), /* account for final jump of three at end */
                    2 => Some((0, 1)),
                    3 => Some((0, 2)),
                    _ => None,
                }
            }
        }

        assert!(!adapters.is_empty());

        let mut i = 0;
        while adapters[i] <= output + 3 {
            let value = adapters.remove(i);
            match joltage_summary_stat(value, adapters) {
                Some((o,t)) => {
                    if value - output == 1 {
                        return Some((o + 1, t));
                    } else if value - output == 3 {
                        return Some((o, t + 1));
                    } else {
                        return Some((o, t));
                    }
                },
                None => {
                    adapters.insert(i, value);
                    if i < adapters.len() - 1 {
                        i += 1;
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
        None
    }

    #[test]
    fn test1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();
        assert_eq!(joltage_summary_stat(0, &mut adapters), Some((7,5)));
    }

    #[test]
    fn test2() {
        let mut adapters = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 
        45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];

        adapters.sort();
        assert_eq!(joltage_summary_stat(0, &mut adapters), Some((22,10)));
    }

}