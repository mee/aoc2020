use std::collections::HashMap;
use std::str::FromStr;

pub fn day19() {
    let (rules, inputs) = parse_input(include_str!("19.input"));
    println!(
        "{} of the inputs are valid",
        count_valid_inputs(&rules, inputs)
    );
}

type Rules = HashMap<usize, Rule>;

#[derive(Debug)]
enum Rule {
    Char(char),
    Conj(Vec<usize>),
    Disj(Vec<usize>, Vec<usize>),
}

#[derive(Debug)]
struct RuleParseError;
impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(s: &'_ str) -> Result<Self, Self::Err> {
        if let Some(pi) = s.find("|") {
            let lh = s[0..pi]
                .trim()
                .split(" ")
                .map(|r| r.parse::<usize>().expect("Unable to parse LHS of |"))
                .collect::<Vec<usize>>();
            let rh = s[pi + 2..]
                .trim()
                .split(" ")
                .map(|r| r.parse::<usize>().expect("Unable to parse RHS of |"))
                .collect::<Vec<usize>>();
            return Ok(Rule::Disj(lh, rh));
        } else if s.starts_with("\"") {
            return Ok(Rule::Char(s.chars().nth(1).unwrap()));
        } else {
            return Ok(Rule::Conj(
                s.split(" ")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            ));
        }
    }
}

fn count_valid_inputs(rules: &Rules, inputs: Vec<&str>) -> usize {
    let mut valid_count = 0;
    let mut total_count = 0;
    for input in inputs {
        let mut i = 0;
        if evaluate_rule_prefix(0, rules, input, &mut i) && i == input.len() {
            valid_count += 1;
        }
        total_count += 1;
    }
    valid_count
}

// returns true if prefix matches
fn evaluate_rule_prefix(rule_num: usize, rules: &Rules, s: &str, i: &mut usize) -> bool {
    assert!(rule_num < rules.len());
    let rule = &rules.get(&rule_num).unwrap();
    /*
    println!(
        "{}{:>3}: {:?} applied to {:<10?}",
        "   ".repeat(*i),
        rule_num,
        rule,
        &s[*i..]
    );
    */
    let ret = match rule {
        Rule::Char(c) => {
            if s.chars().nth(*i).unwrap() == *c {
                *i += 1;
                true
            } else {
                false
            }
        }
        Rule::Disj(lh, rh) => {
            let j = i.clone();

            if lh
                .iter()
                .all(|&r| evaluate_rule_prefix(r, rules, &s, &mut *i))
            {
                true
            } else {
                *i = j; // <sound of record scratching>
                rh.iter()
                    .all(|&r| evaluate_rule_prefix(r, rules, &s, &mut *i))
            }
        }
        Rule::Conj(rs) => rs
            .iter()
            .all(|&r| evaluate_rule_prefix(r, rules, &s, &mut *i)),
    };
    ret
}

fn parse_input(s: &str) -> (Rules, Vec<&str>) {
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    let mut lines = s.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let colon = line.find(":").unwrap();
        let rule_num = line[0..colon].parse::<usize>().unwrap();
        let rule = line[colon + 2..].parse::<Rule>().unwrap();
        rules.insert(rule_num, rule);
    }

    (rules, lines.collect::<Vec<&str>>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn pp_rules(rules: &Rules) {
        println!("Rules:");
        rules
            .iter()
            .enumerate()
            .for_each(|(ri, r)| println!("{:>3}: {:?}", ri, r));
        println!();
    }

    const INPUT1: &str = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

"#;

    const INPUT2: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn test_simple() {
        let (rules, _) = parse_input(INPUT1);
        pp_rules(&rules);
        assert!(evaluate_rule_prefix(0, &rules, "aab", &mut 0));
        assert!(evaluate_rule_prefix(0, &rules, "aba", &mut 0));
        assert!(!evaluate_rule_prefix(0, &rules, "bbb", &mut 0));
        assert!(!evaluate_rule_prefix(0, &rules, "bab", &mut 0));
    }

    #[test]
    fn test_parse_1() {
        let (_rules, inputs) = parse_input(INPUT2);
        assert_eq!(inputs.len(), 5);
        assert_eq!(inputs.first().unwrap(), &"ababbb");
        assert_eq!(inputs.last().unwrap(), &"aaaabbb");
    }

    #[test]
    fn test1() {
        let (rules, _) = parse_input(INPUT2);
        pp_rules(&rules);
        assert!(evaluate_rule_prefix(0, &rules, "ababbb", &mut 0));
    }

    #[test]
    fn test2() {
        let (rules, _) = parse_input(INPUT2);
        pp_rules(&rules);
        assert!(evaluate_rule_prefix(0, &rules, "abbbab", &mut 0));
    }

    #[test]
    fn test3() {
        let (rules, _) = parse_input(INPUT2);
        pp_rules(&rules);
        assert!(!evaluate_rule_prefix(0, &rules, "bababa", &mut 0));
    }

    #[test]
    fn test4() {
        let (rules, _) = parse_input(INPUT2);
        pp_rules(&rules);
        assert!(!evaluate_rule_prefix(0, &rules, "aaabbb", &mut 0));
    }

    #[test]
    fn test5() {
        let (rules, _) = parse_input(INPUT2);
        pp_rules(&rules);
        let mut i = 0;
        let s = "aaaabbb";
        assert!(!(evaluate_rule_prefix(0, &rules, s, &mut i) && dbg!(i) == s.len()));
    }

    #[test]
    fn test_batch_1() {
        let (rules, inputs) = parse_input(INPUT2);
        assert_eq!(count_valid_inputs(&rules, inputs), 2);
    }
}
