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
        "11" => day11::day11(),
        "12" => day12::day12(),
        "13" => day13::day13(),
        "14" => day14::day14(),
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
            Command { index, op }
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
                println!(
                    "Sum of smallest and largest in running sum to invalid number: {}",
                    min + max
                );
            }
        }
    }
}

// day 10
mod day10 {
    pub fn day10() {
        let adapters = include_str!("10.input")
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if let Some(stats) = joltage_summary_stat(0, &mut adapters.clone()) {
            println!(
                "The product of one-jumps and three-jumps is {}",
                stats.0 * stats.1
            );
        } else {
            println!("No valid combinations of adapters found");
        }

        println!(
            "There are {} possible ways to adapt",
            joltage_combo_count(&mut adapters.clone())
        );
    }

    // requires `adapters` be sorted - Vec::is_sorted() eXperimental in stable...
    pub fn joltage_summary_stat(
        output: usize,
        adapters: &mut Vec<usize>,
    ) -> Option<(usize, usize)> {
        adapters.sort();
        if adapters.len() == 1 {
            if output > adapters[0] {
                return None;
            } else {
                return match adapters[0] - output {
                    1 => Some((1, 1)), /* account for final jump of three at end */
                    2 => Some((0, 1)),
                    3 => Some((0, 2)),
                    _ => None,
                };
            }
        }

        assert!(!adapters.is_empty());

        let mut i = 0;
        while adapters[i] <= output + 3 {
            let value = adapters.remove(i);
            match joltage_summary_stat(value, adapters) {
                Some((o, t)) => {
                    if value - output == 1 {
                        return Some((o + 1, t));
                    } else if value - output == 3 {
                        return Some((o, t + 1));
                    } else {
                        return Some((o, t));
                    }
                }
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

    pub fn joltage_combo_count(adapters: &mut Vec<usize>) -> usize {
        adapters.sort();
        adapters.insert(0, 0);
        let mut counts: Vec<usize> = Vec::with_capacity(adapters.len());
        for _ in 0..adapters.len() {
            counts.push(0);
        }
        counts[0] = 1; // start with one path to zero

        let mut i = 1;
        while i < adapters.len() {
            /* counts[i] is the sum of the counts of the k previous, accessible counts,
            sum(counts[i-k..i-1]) */
            let mut j = i - 1;
            loop {
                if adapters[i] - adapters[j] > 3 {
                    break;
                }
                counts[i] += counts[j];
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            i += 1;
        }
        counts[counts.len() - 1]
    }

    #[test]
    fn test1a() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();
        assert_eq!(joltage_summary_stat(0, &mut adapters), Some((7, 5)));
    }

    #[test]
    fn test2a() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();
        assert_eq!(joltage_combo_count(&mut adapters), 8);
    }

    #[test]
    fn test1b() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        adapters.sort();
        assert_eq!(joltage_summary_stat(0, &mut adapters), Some((22, 10)));
    }

    #[test]
    fn test2b() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        adapters.sort();
        assert_eq!(joltage_combo_count(&mut adapters), 19208);
    }
}

mod day11 {
    use std::{cmp::min, fmt, str::FromStr};

    #[derive(Clone, Debug, Copy, PartialEq)]
    enum SeatStatus {
        Empty,
        Occupied,
        Floor,
    }

    impl fmt::Display for SeatStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SeatStatus::Floor => write!(f, "."),
                SeatStatus::Occupied => write!(f, "#"),
                SeatStatus::Empty => write!(f, "L"),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct SeatMap {
        width: usize,
        height: usize,
        map: Vec<Vec<SeatStatus>>,
    }

    impl FromStr for SeatStatus {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "L" => Ok(SeatStatus::Empty),
                "." => Ok(SeatStatus::Floor),
                "#" => Ok(SeatStatus::Occupied),
                _ => Err(ParseError),
            }
        }
    }

    // indices are height (rows), width (cols)
    impl SeatMap {
        fn new(width: usize, height: usize) -> Self {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(SeatStatus::Empty)
            }
            let mut map = Vec::with_capacity(height);
            for _ in 0..height {
                map.push(row.clone());
            }
            Self { width, height, map }
        }

        fn update(self: &mut Self, r: usize, c: usize, status: SeatStatus) {
            *(self.map.get_mut(r).unwrap().get_mut(c).unwrap()) = status;
        }

        fn get(self: &Self, r: usize, c: usize) -> Option<SeatStatus> {
            if r < self.height && c < self.width {
                Some(self.map[r][c])
            } else {
                None
            }
        }

        fn adjacents(self: &Self, r: usize, c: usize) -> Vec<SeatStatus> {
            let rlb = if r == 0 { 0 } else { r - 1 };
            let clb = if c == 0 { 0 } else { c - 1 };
            let rub = min(r + 1, self.height - 1);
            let cub = min(c + 1, self.width - 1);

            let mut adjacents: Vec<SeatStatus> = Vec::new();
            for ri in rlb..=rub {
                for ci in clb..=cub {
                    if ri == r && ci == c {
                        continue;
                    }
                    adjacents.push(self.get(ri, ci).unwrap());
                }
            }
            adjacents
        }

        fn seats_in_los(self: &Self, r: usize, c: usize) -> Vec<SeatStatus> {
            let mut dirs: Vec<(isize, isize)> = vec![];
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    dirs.push((i, j));
                }
            }

            let mut seats: Vec<SeatStatus> = Vec::new();
            'dir: for dir in dirs {
                let mut pos = (r as isize, c as isize);
                loop {
                    if let Some(new_r) = pos.0.checked_add(dir.0) {
                        if let Some(new_c) = pos.1.checked_add(dir.1) {
                            if let Some(status) = self.get(new_r as usize, new_c as usize) {
                                if status != SeatStatus::Floor {
                                    seats.push(status);
                                    continue 'dir;
                                } else {
                                    pos = (new_r, new_c);
                                    continue;
                                }
                            } else {
                                continue 'dir;
                            }
                        } else {
                            continue 'dir;
                        }
                    } else {
                        continue 'dir;
                    }
                }
            }
            seats
        }

        fn update_seat(self: &Self, r: usize, c: usize, adj: bool) -> SeatStatus {
            let cur = self.get(r, c).unwrap();

            if cur == SeatStatus::Floor {
                return SeatStatus::Floor;
            }

            let adj_count = if adj {
                self.adjacents(r, c)
            } else {
                self.seats_in_los(r, c)
            }
            .iter()
            .filter(|x| **x == SeatStatus::Occupied)
            .count();

            let adj_count_too_high = adj_count >= if adj { 4 } else { 5 };

            if cur == SeatStatus::Empty && adj_count == 0 {
                return SeatStatus::Occupied;
            } else if cur == SeatStatus::Occupied && adj_count_too_high {
                return SeatStatus::Empty;
            } else {
                return cur;
            }
        }

        fn update_map(self: &Self, adj: bool) -> Self {
            let mut new = self.clone();
            for r in 0..self.height {
                for c in 0..self.width {
                    let new_status = self.update_seat(r, c, adj);
                    new.update(r, c, new_status);
                }
            }
            new
        }

        pub fn occupied(self: &Self) -> usize {
            let mut count = 0;
            for row in &self.map {
                for val in row {
                    if *val == SeatStatus::Occupied {
                        count += 1;
                    }
                }
            }
            count
        }

        pub fn finally_occupied(self: &SeatMap, adj: bool) -> usize {
            let mut curr = self.clone();
            let mut next = self.update_map(adj);

            while next != curr {
                curr = next;
                next = curr.update_map(adj);
            }

            next.occupied()
        }
    }

    #[derive(Debug)]
    struct ParseError;
    impl FromStr for SeatMap {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines = s.lines().collect::<Vec<&str>>();
            let width = lines[0].len();
            let height = lines.len();
            let mut map = SeatMap::new(width, height);
            for (i, line) in lines.iter().enumerate() {
                for j in 0..line.len() {
                    let c: &str = line.get(j..j + 1).unwrap();
                    map.update(i, j, c.parse::<SeatStatus>().unwrap());
                }
            }
            Ok(map)
        }
    }

    impl fmt::Display for SeatMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for i in 0..self.height {
                for j in 0..self.width {
                    write!(f, "{}", self.get(i, j).unwrap())?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")
        }
    }

    pub fn day11() {
        let input = include_str!("11.input");
        let map = input.parse::<SeatMap>().unwrap();
        println!(
            "Using adjacency, {} seats were finally occupied",
            map.finally_occupied(true)
        );
        println!(
            "Using visibility, {} seats were finally occupied",
            map.finally_occupied(false)
        );
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const INPUT1: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        const INPUT2: &str = r"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

        const INPUT3: &str = r".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";

        #[test]
        fn parse() {
            let map = INPUT1.parse::<SeatMap>().unwrap();
            assert_eq!(map.width, 10);
            assert_eq!(map.height, 10);
            assert_eq!(map.occupied(), 0);
        }

        #[test]
        fn update_seat() {
            let map1 = INPUT1.parse::<SeatMap>().unwrap();
            assert_eq!(map1.update_seat(0, 0, true), SeatStatus::Occupied);
        }

        #[test]
        fn update_seat2() {
            let map2 = INPUT2.parse::<SeatMap>().unwrap();
            println!("{}", &map2);
            assert_eq!(map2.update_seat(1, 2, true), SeatStatus::Empty);
        }

        #[test]
        fn finally_occupied() {
            let map = INPUT1.parse::<SeatMap>().unwrap();
            assert_eq!(map.finally_occupied(true), 37);
        }

        #[test]
        fn none_visible() {
            let map = INPUT3.parse::<SeatMap>().unwrap();
            assert!(map.seats_in_los(3, 3).is_empty());
        }

        #[test]
        fn finally_occupied_visible() {
            let map = INPUT1.parse::<SeatMap>().unwrap();
            assert_eq!(map.finally_occupied(false), 26);
        }
    }
}

mod day12 {
    use std::str::FromStr;

    pub fn day12() {
        let commands = include_str!("12.input")
            .lines()
            .map(|l| l.parse::<Command>().unwrap())
            .collect();

        println!("The Manhattan distance is {}", distance(&commands));
        println!(
            "The manhattan distance is {} (waypoint method)",
            distance2(&commands)
        );
    }

    #[derive(Debug, PartialEq)]
    pub enum Command {
        N(isize),
        S(isize),
        E(isize),
        W(isize),
        L(isize),
        R(isize),
        F(isize),
    }

    type State = (isize, isize, isize);
    type WaypointState = (isize, isize, isize, isize); // x,y of ship, waypoint offset

    #[derive(Debug, Clone, Copy)]
    pub struct ParseError;

    impl FromStr for Command {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let action = &s[0..1];
            let value = &s[1..].parse::<isize>().unwrap();
            match action {
                "N" => Ok(Command::N(*value)),
                "S" => Ok(Command::S(*value)),
                "E" => Ok(Command::E(*value)),
                "W" => Ok(Command::W(*value)),
                "L" => Ok(Command::L(*value)),
                "R" => Ok(Command::R(*value)),
                "F" => Ok(Command::F(*value)),
                _ => Err(ParseError),
            }
        }
    }

    fn distance(cmds: &Vec<Command>) -> usize {
        let mut state: State = (0, 0, 0); // x, y, heading; 0 degrees is East
        for cmd in cmds {
            state = apply(&cmd, state);
        }
        (state.0.abs() + state.1.abs()) as usize
    }

    fn distance2(cmds: &Vec<Command>) -> usize {
        let mut state: WaypointState = (0, 0, 10, 1); // x, y of ship, waypoint offset
        for cmd in cmds {
            state = apply2(&cmd, state);
        }
        (state.0.abs() + state.1.abs()) as usize
    }

    pub fn apply(cmd: &Command, state: State) -> State {
        match cmd {
            Command::N(d) => (state.0, state.1 + d, state.2),
            Command::S(d) => (state.0, state.1 - d, state.2),
            Command::E(d) => (state.0 + d, state.1, state.2),
            Command::W(d) => (state.0 - d, state.1, state.2),
            Command::L(d) => (state.0, state.1, (state.2 + (360 - d)) % 360), /* could also solve this by varying d over (-180, 180) instead of (0, 360) */
            Command::R(d) => (state.0, state.1, (state.2 + d) % 360),
            Command::F(d) => {
                let new_cmd = match state.2 {
                    0 => Command::E(*d),
                    90 => Command::S(*d),
                    180 => Command::W(*d),
                    270 => Command::N(*d),
                    x => panic!("no way {}", x),
                };
                apply(&new_cmd, state)
            }
        }
    }

    /* R = 90 so rotation matrix is
     * [ cos 90 ; -sin 90 ] = [ 0 ; -1 ]
     * [ sin 90 ; cos 90  ]   [ 1 ;  0 ]
     *
     * L = -90 so rotation matrix is
     * [ 0  ; 1 ]
     * [ -1 ; 0 ]
     *
     * r = [ x y ];
     * Rr = [ (cos T)x - (sin T)y ; (sin T)x + (cos T)y ]
     *
     */

    fn rotate_left(d: isize, state: &mut WaypointState) -> WaypointState {
        for _ in 0..(d / 90).abs() {
            *state = (state.0, state.1, -state.3, state.2);
        }
        *state
    }

    fn rotate_right(d: isize, state: &mut WaypointState) -> WaypointState {
        for _ in 0..(d / 90).abs() {
            *state = (state.0, state.1, state.3, -state.2);
        }
        *state
    }

    // part 2
    pub fn apply2(cmd: &Command, state: WaypointState) -> WaypointState {
        match cmd {
            Command::N(d) => (state.0, state.1, state.2, state.3 + d),
            Command::S(d) => (state.0, state.1, state.2, state.3 - d),
            Command::E(d) => (state.0, state.1, state.2 + d, state.3),
            Command::W(d) => (state.0, state.1, state.2 - d, state.3),
            Command::L(d) => rotate_left(*d, &mut state.clone()),
            Command::R(d) => rotate_right(*d, &mut state.clone()),
            Command::F(d) => (
                state.0 + d * state.2,
                state.1 + d * state.3,
                state.2,
                state.3,
            ),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const INPUT1: &str = "F10
N3
F7
R90
F11";

        #[test]
        fn parse_example() {
            let commands: Vec<Command> = INPUT1
                .lines()
                .map(|l| l.parse::<Command>().unwrap())
                .collect();
            assert_eq!(commands[0], Command::F(10));
            assert_eq!(commands[1], Command::N(3));
            assert_eq!(commands[2], Command::F(7));
            assert_eq!(commands[3], Command::R(90));
            assert_eq!(commands[4], Command::F(11));
        }

        #[test]
        fn example() {
            let commands: Vec<Command> = INPUT1
                .lines()
                .map(|l| l.parse::<Command>().unwrap())
                .collect();
            assert_eq!(distance(&commands), 25);
        }

        #[test]
        fn example2() {
            let commands: Vec<Command> = INPUT1
                .lines()
                .map(|l| l.parse::<Command>().unwrap())
                .collect();
            assert_eq!(distance2(&commands), 286);
        }

        #[test]
        fn rotate() {
            assert_eq!(rotate_right(90, &mut (0, 0, 1, 1)), (0, 0, 1, -1));
            assert_eq!(rotate_right(180, &mut (0, 0, 1, 1)), (0, 0, -1, -1));
            assert_eq!(rotate_right(270, &mut (0, 0, 1, 1)), (0, 0, -1, 1));
            assert_eq!(rotate_left(90, &mut (0, 0, 1, 1)), (0, 0, -1, 1));
            assert_eq!(rotate_left(180, &mut (0, 0, 1, 1)), (0, 0, -1, -1));
            assert_eq!(rotate_left(270, &mut (0, 0, 1, 1)), (0, 0, 1, -1));
        }
    }
}

mod day13 {
    use num::Integer;

    pub fn day13() {
        let notes = parse(include_str!("13.input"));
        let (bus, earliest) = find_earliest(notes.0, notes.1.clone());

        println!(
            "product of bus number and wait time is {}",
            bus * (earliest - notes.0)
        );

        println!(
            "sequential bus arrivals first happen at {}",
            find_sequential(notes.1, 10000000000000)
        );
    }

    fn parse(s: &str) -> (usize, Vec<usize>) {
        let lines = s.lines().collect::<Vec<&str>>();
        let lb = lines[0].parse::<usize>().unwrap();
        let buses = lines[1]
            .split(',')
            .map(|s| if s == "x" { "0" } else { s })
            .map(|b| b.parse::<usize>().unwrap())
            .collect();
        (lb, buses)
    }

    fn find_earliest(lb: usize, buses: Vec<usize>) -> (usize, usize) {
        buses
            .iter()
            .filter(|x| **x > 0)
            .map(|t| (*t, ((lb / t) + 1) * t))
            .fold((0, usize::MAX), |a, b| if a.1 < b.1 { a } else { b })
    }

    fn find_sequential(buses: Vec<usize>, start: usize) -> usize {
        let mut i = ((start / buses[0]) + 1) * buses[0];
        let mut jump = buses[0];
        'chance: loop {
            for j in 1..buses.len() {
                if buses[j] == 0 {
                    print_bus(buses[j], j, i);
                    continue;
                }
                if (i + j) % buses[j] != 0 {
                    i += jump;
                    continue 'chance;
                }
                jump = jump.lcm(&buses[j]);
                print_bus(buses[j], j, i);
            }
            return i;
        }
    }

    fn print_bus(bus: usize, offset: usize, start: usize) {
        let bus_char = if bus == 0 { "-" } else { "D" };
        if offset == 1 {
            println!("{}\tD", start);
        }
        print!("{}\t", start + offset);
        for _ in 0..offset {
            print!(" ");
        }
        println!("{}", bus_char);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const INPUT1: &str = "939
7,13,x,x,59,x,31,19";

        const INPUT2: &str = "3417
17,x,13,19";

        const INPUT3: &str = "754018
67,7,59,61";

        const INPUT4: &str = "779210
67,x,7,59,61";

        const INPUT5: &str = "1261476
67,7,x,59,61";

        const INPUT6: &str = "1202161486
1789,37,47,1889";

        #[test]
        fn test_parse() {
            assert_eq!(parse(INPUT1), (939, vec![7, 13, 0, 0, 59, 0, 31, 19]));
        }

        #[test]
        fn example() {
            let notes = parse(INPUT1);
            assert_eq!(find_earliest(notes.0, notes.1), (59, 944));
        }

        #[test]
        fn example_sequential1() {
            let notes = parse(INPUT1);
            assert_eq!(find_sequential(notes.1, 0), 1068781);
        }

        #[test]
        fn example_sequential2() {
            let notes = parse(INPUT2);
            assert_eq!(find_sequential(notes.1, 0), notes.0);
        }

        #[test]
        fn example_sequential3() {
            let notes = parse(INPUT3);
            assert_eq!(find_sequential(notes.1, 0), notes.0);
        }

        #[test]
        fn example_sequential4() {
            let notes = parse(INPUT4);
            assert_eq!(find_sequential(notes.1, 0), notes.0);
        }

        #[test]
        fn example_sequential5() {
            let notes = parse(INPUT5);
            assert_eq!(find_sequential(notes.1, 0), notes.0);
        }

        #[test]
        fn example_sequential6() {
            let notes = parse(INPUT6);
            assert_eq!(find_sequential(notes.1, 0), notes.0);
        }
    }
}

mod day14 {
    use num_traits::pow;
    use std::collections::HashMap;

    /*
     * OK, let's represent this "mask" as two masks: the zeros (z) and the ones (o)
     * a mask of xx1x0 would be decomposed into
     *         z 11110    <-- 0 if mask is 0 at that position, otherwise 1
     *         o 00100    <-- 1 if mask is 1 at that position, otherwise 0
     *
     * That way, I can modify value 01101 by doing x' = (x & z) | o
     *    x 01101
     *    z 11110 (&)
     *      01100
     *    o 00100 (|)
     *   x' 01100
     */
    pub fn day14() {
        let input = include_str!("14.input");
        println!("final sum: {}", execute(&input));
        println!("final sum (part 2): {}", execute2(&input));
    }

    fn parse_mask(mask: &str) -> (u64, u64) {
        let mut z = u64::MAX;
        let mut o = 0 as u64;
        let offset = mask.len() - 1;
        for (i, c) in mask.chars().enumerate() {
            if c == '1' {
                o |= 1 << (offset - i); // set o at pos i to 1
            } else if c == '0' {
                z &= u64::MAX - (1 << (offset - i)); // set z at pos i to 0
            }
        }
        (z, o)
    }

    // let's brute force!
    fn execute2(s: &str) -> usize {
        let mut mem: HashMap<String, usize> = HashMap::new();
        let mut mask = "bad mask";
        for line in s.lines() {
            match &line[..=2] {
                "mas" => {
                    mask = &line[7..];
                }
                "mem" => {
                    let lbindex = line.find(']').unwrap();
                    let loc = &line[4..lbindex];
                    let val = line[lbindex + 4..].parse::<usize>().unwrap();
                    let expanded_addrs = expand_addr(&apply_mask(loc, &mask));
                    for addr in expanded_addrs {
                        mem.insert(addr, val);
                    }
                }
                s => panic!("ahhh! {}", s),
            }
        }

        mem.values().fold(0, |a, b| a + b)
    }

    fn expand_addr(m: &str) -> Vec<String> {
        if m.contains('X') {
            let b0 = m.clone().replacen("X", "0", 1);
            let b1 = m.clone().replacen("X", "1", 1);
            let mut ret0 = expand_addr(&b0);
            let mut ret1 = expand_addr(&b1);
            ret0.append(&mut ret1);
            ret0
        } else {
            vec![String::from(m)]
        }
    }

    // bad code bad code wee ooo wee ooo wee ooo
    fn apply_mask<'a>(l: &'a str, m: &'a str) -> String {
        let mut ll = String::new();
        let mut lo = l.parse::<usize>().unwrap();
        while lo > 0 {
            ll.insert(
                0,
                match lo % 2 {
                    0 => '0',
                    1 => '1',
                    _ => panic!("bad input {}", l),
                },
            );
            lo /= 2;
        }
        ll = format!("{:0>36}", ll);

        let ret = ll
            .chars()
            .zip(m.chars())
            .map(|(lc, mc)| match (lc, mc) {
                (x, '0') => x,
                (_, '1') => '1',
                (_, 'X') => 'X',
                _ => panic!("bad mask bit {} or loc bit {}", mc, lc),
            })
            .collect::<String>();
        ret
    }

    fn execute(s: &str) -> u64 {
        let mut mask: (u64, u64) = (0, 0);
        let mut mem: HashMap<u64, u64> = HashMap::new();
        for line in s.lines() {
            match &line[..=2] {
                "mas" => {
                    mask = parse_mask(&line[7..].trim());
                }
                "mem" => {
                    let lbindex = line.find(']').unwrap();
                    let loc = line[4..lbindex].parse::<u64>().unwrap();
                    let val = line[lbindex + 4..].parse::<u64>().unwrap();
                    mem.insert(loc, val & mask.0 | mask.1);
                }
                s => panic!("ahhh! {}", s),
            }
        }
        mem.values().fold(0 as u64, |v1, v2| v1 + v2)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const INPUT1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        const INPUT2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        #[test]
        fn test_execute() {
            let val = execute(&INPUT1);
            assert_eq!(val, 165);
        }

        #[test]
        fn test_execute2() {
            println!("{}", &INPUT2);
            let val = execute2(&INPUT2);
            assert_eq!(val, 208);
        }

        #[test]
        fn test_expand_addr() {
            assert_eq!(expand_addr("X0X"), vec!["000", "001", "100", "101"]);
        }
    }
}
