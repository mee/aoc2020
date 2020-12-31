use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn day16() {
    let (constraints, my_ticket, mut tickets) = parse_input(include_str!("16.input")).unwrap();

    println!(
        "The invalid fields sum to {}",
        find_invalid_ticket_field_sum(&mut tickets, &constraints)
    );

    tickets.retain(|t| t.invalid_ticket_values(&constraints).is_empty());
    tickets
        .iter_mut()
        .for_each(|t| t.assign_field_labels(&constraints));

    let mut i = 0;
    while !tickets.iter().all(|t| t.is_valid()) {
        refine_ticket_labels(&mut tickets);
        print!(".");
        i += 1;
    }
    println!("\nsolved after {} iterations", i);

    let final_labels = tickets.get(0).unwrap().get_first_field_labels();

    let mut product: usize = 1;
    for (i, label) in final_labels.iter().enumerate() {
        if label.starts_with("departure") {
            product *= my_ticket.fields.get(i).unwrap().0;
        }
    }

    println!("The product of my departure fields is {}", product);
}

fn find_invalid_ticket_field_sum(
    tickets: &mut Vec<Ticket>,
    constraints: &Vec<FieldConstraint>,
) -> usize {
    tickets.iter_mut().fold(0, |s, t| {
        s + t.invalid_ticket_values(&constraints).iter().sum::<usize>()
    })
}

/*
 * strategy:
 * parse the constraints
 * iterate over tickets
 * for each field in each ticket
 * annotate the field with constraints it matches
 * if it matches only one constraint, remove that constraint from all previous fields &
 *   do not apply it to subsequent ones
 *
 *
 * concern: possible ambiguity in which fields are invalid?
 */

#[derive(Debug)]
struct ParseInputError;
fn parse_input(s: &str) -> Result<(Vec<FieldConstraint>, Ticket, Vec<Ticket>), ParseInputError> {
    let mut constraints: Vec<FieldConstraint> = Vec::new();
    let mut tickets: Vec<Ticket> = Vec::new();
    let lines = s.lines().collect::<Vec<&str>>();
    let mut i = 0;
    while lines[i] != "" {
        constraints.push(lines[i].parse::<FieldConstraint>().unwrap());
        i += 1;
    }
    i += 2;
    let my_ticket = lines[i].parse::<Ticket>().unwrap();
    i += 3;
    while i < lines.len() {
        tickets.push(lines[i].parse::<Ticket>().unwrap());
        i += 1;
    }
    Ok((constraints, my_ticket, tickets))
}

// wee woo wee woo bad code
// assumes all tickets are valid
fn refine_ticket_labels(tickets: &mut Vec<Ticket>) {
    // the labels that exist for every ticket at a given field index
    // field index -> { labels }
    let mut labels: HashMap<usize, HashSet<String>> = HashMap::new();

    // reduce in column-wise
    let first_ticket = tickets.get(0).unwrap();
    for (fi, f) in first_ticket.fields.iter().enumerate() {
        labels.insert(fi, f.1.clone());
    }
    for ti in 1..tickets.len() {
        let t = tickets.get(ti).unwrap();
        for (fi, f) in t.fields.iter().enumerate() {
            labels.get_mut(&fi).unwrap().retain(|e| f.1.contains(e));
        }
    }

    // update tickets
    for t in tickets.iter_mut() {
        t.update_labels(&labels);
    }

    for t in tickets.iter_mut() {
        t.refine_labels();
    }
}

#[derive(Debug, PartialEq)]
struct FieldConstraint {
    label: String,
    ranges: Vec<(usize, usize)>,
}

impl FieldConstraint {
    fn matches(self: &Self, val: usize) -> bool {
        self.ranges
            .iter()
            .any(|range| range.0 <= val && val <= range.1)
    }
}

#[derive(Debug)]
struct ConstraintParseError;
impl FromStr for FieldConstraint {
    type Err = ConstraintParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges: Vec<(usize, usize)> = Vec::new();
        if let Some(colon_pos) = s.find(':') {
            let label = String::from(&s[0..colon_pos]);
            for range_str in s[colon_pos + 2..].split(" or ") {
                let dash_pos = range_str.find('-').unwrap();
                let lb = range_str
                    .get(0..dash_pos)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let ub = range_str
                    .get(dash_pos + 1..)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                ranges.push((lb, ub));
            }
            return Ok(FieldConstraint { label, ranges });
        }
        return Err(ConstraintParseError);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    // [ value -> [ possible labels ] ]
    fields: Vec<(usize, HashSet<String>)>,
}

impl Ticket {
    fn get_first_field_labels(self: &Self) -> Vec<String> {
        assert!(self.is_valid());
        self.fields
            .iter()
            .map(|f| f.1.iter().next().unwrap())
            .cloned()
            .collect::<Vec<String>>()
    }

    fn invalid_ticket_values(self: &Self, constraints: &Vec<FieldConstraint>) -> Vec<usize> {
        self.fields
            .iter()
            .filter(|f| !constraints.iter().any(|c| c.matches(f.0)))
            .map(|f| f.0)
            .collect()
    }

    fn assign_field_labels(self: &mut Self, constraints: &Vec<FieldConstraint>) {
        for f in self.fields.iter_mut() {
            for c in constraints {
                if c.matches(f.0) {
                    f.1.insert(c.label.clone()); // wee-woo wee-woo
                }
            }
        }
    }

    fn is_valid(self: &Self) -> bool {
        self.fields.iter().all(|f| f.1.len() == 1)
    }

    fn refine_labels(self: &mut Self) {
        let mut label_pos = HashSet::new();
        for f in self.fields.iter() {
            if f.1.len() == 1 {
                let taken = f.1.iter().find(|_| true).unwrap().clone();
                label_pos.insert(taken);
            }
        }

        self.fields.iter_mut().for_each(|f| {
            if f.1.len() > 1 {
                f.1.retain(|l| !label_pos.contains(l));
            }
            if f.1.len() == 1 {
                label_pos.insert(f.1.iter().find(|_| true).unwrap().clone());
            }
        });
    }

    fn update_labels(self: &mut Self, labels: &HashMap<usize, HashSet<String>>) {
        for (fi, f) in self.fields.iter_mut().enumerate() {
            f.1 = labels.get(&fi).unwrap().clone();
        }
    }
}

#[derive(Debug)]
struct TicketParseError;
impl FromStr for Ticket {
    type Err = TicketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            fields: s
                .split(',')
                .map(|vs| (vs.parse::<usize>().unwrap(), HashSet::new()))
                .collect::<Vec<(usize, HashSet<String>)>>(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_parse_constraint() {
        assert_eq!(
            "row: 6-11 or 33-44".parse::<FieldConstraint>().unwrap(),
            FieldConstraint {
                label: String::from("row"),
                ranges: vec![(6, 11), (33, 44)],
            }
        );
    }

    #[test]
    fn test_parse_ticket() {
        let set = HashSet::new();
        assert_eq!(
            "7,1,14".parse::<Ticket>().unwrap(),
            Ticket {
                fields: vec![(7, set.clone()), (1, set.clone()), (14, set.clone())]
            }
        );
    }

    #[test]
    fn test_parse_input() {
        let parsed_input = parse_input(INPUT1).unwrap();
        assert_eq!(parsed_input.0.len(), 3); // 3 constraints
        assert_eq!(parsed_input.2.len(), 4); // 4 other tickets
    }

    #[test]
    fn test_part_1() {
        let (constraints, _, mut tickets) = parse_input(include_str!("16.input")).unwrap();
        assert_eq!(
            find_invalid_ticket_field_sum(&mut tickets, &constraints),
            21071
        );
    }

    #[test]
    fn test_ticket_invalid_values() {
        let set = HashSet::new();
        let ticket = Ticket {
            fields: vec![(5, set.clone())],
        };
        let constraints = vec![FieldConstraint {
            label: String::from("label"),
            ranges: vec![(4, 6)],
        }];
        assert_eq!(ticket.invalid_ticket_values(&constraints).len(), 0);
    }
}
