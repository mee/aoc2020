use std::error::Error;
use std::str::FromStr;
pub fn day18() {
    let mut sum = 0;
    let input = include_str!("18.input");
    for line in input.lines() {
        let res = eval_str(line).unwrap();
        println!("{:>12} = {}", res, line);
        sum += res;
    }
    println!("The sum of all results is {}", sum);

    sum = 0;
    for line in input.lines() {
        let res = eval_adv(line);
        println!("{:>12} = {}", res, line);
        sum += res;
    }
    println!("Using advanced math, the sum of all results is {}", sum);
}

#[derive(Debug, PartialEq)]
struct EvalError;
#[derive(Debug)]
struct OpParseError;

impl Error for EvalError {}
impl Error for OpParseError {}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for OpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Multiply,
}

impl Op {
    fn apply(self: &Self, a: isize, b: isize) -> isize {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
        }
    }
}

impl FromStr for Op {
    type Err = OpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Op::Multiply),
            "+" => Ok(Op::Add),
            _ => Err(OpParseError),
        }
    }
}

fn find_closing_paren(s: &str, pi_lh: usize) -> usize {
    s[pi_lh..]
        .chars()
        .scan(0, |pc, c| {
            *pc = match c {
                '(' => *pc + 1,
                ')' => *pc - 1,
                _ => *pc,
            };
            Some(*pc)
        })
        .take_while(|&e| e > 0)
        .collect::<Vec<usize>>()
        .len()
        + pi_lh
}

fn get_next_expr(s: &str) -> Result<(isize, &str), EvalError> {
    if &s[0..=0] == "(" {
        let pi_rh = find_closing_paren(s, 0);
        let (expr_in, remain) = if pi_rh < s.len() - 1 {
            (&s[1..pi_rh], &s[pi_rh + 1..])
        } else {
            (&s[1..s.len() - 1], "")
        };
        return eval_str(expr_in).map(|v| (v, remain));
    } else {
        let val_str = s.chars().take_while(|&c| c != ' ').collect::<String>();
        return val_str
            .parse::<isize>()
            .map_err(|_| EvalError)
            .map(|v| (v, &s[val_str.len()..]));
    }
}

fn get_next_op(s: &str) -> Result<(Op, &str), EvalError> {
    s[1..=1]
        .parse::<Op>()
        .map_err(|_| EvalError)
        .map(|op| (op, &s[3..]))
}

fn eval_str(s: &str) -> Result<isize, EvalError> {
    let (mut cur, mut s) = get_next_expr(s)?;
    while !s.is_empty() {
        let (op, ns) = get_next_op(s)?;
        let (rh, ns) = get_next_expr(ns)?;
        cur = op.apply(cur, rh);
        s = ns;
    }

    Ok(cur)
}

// not sure this will work, because expressions might be dropped w/o non-weak refs to them.
// back-up idea: store non-weak refs to them in a hashset
#[derive(Debug, PartialEq)]
enum AdvExpr {
    Value(isize),
    Add,
    Multiply,
}

fn get_next_expr_adv(s: &str) -> (AdvExpr, &str) {
    if s.starts_with('(') {
        let pi_rh = find_closing_paren(s, 0);
        let expr = eval_adv_expr(&s[1..pi_rh]).unwrap();
        return (expr, &s[pi_rh + 1..]);
    } else if s.starts_with(char::is_numeric) {
        if let Some(si) = s.find(|c: char| !c.is_numeric()) {
            let val = s[0..si].parse::<isize>().unwrap();
            return (AdvExpr::Value(val), &s[si..]);
        } else {
            let val = s[0..].parse::<isize>().unwrap();
            return (AdvExpr::Value(val), "");
        }
    } else {
        return match &s[0..3] {
            " + " => (AdvExpr::Add, &s[3..]),
            " * " => (AdvExpr::Multiply, &s[3..]),
            unk => panic!("unrecognized expression '{}'", unk),
        };
    }
}

fn eval_adv(s: &str) -> isize {
    if let Ok(AdvExpr::Value(v)) = eval_adv_expr(s) {
        return v;
    }
    panic!("Unable to evaluate {} to a value", s);
}

fn eval_adv_expr(s: &str) -> Result<AdvExpr, EvalError> {
    let mut expr_str = s;
    let mut stack: Vec<AdvExpr> = Vec::new();
    loop {
        let (expr, remainder) = get_next_expr_adv(&expr_str);
        stack.push(expr);
        expr_str = remainder;
        if expr_str.is_empty() {
            break;
        }
    }

    while stack.contains(&AdvExpr::Add) {
        let ci = stack.iter().position(|e| *e == AdvExpr::Add).unwrap();
        if let AdvExpr::Value(lh) = stack.remove(ci - 1) {
            stack.remove(ci - 1);
            if let AdvExpr::Value(rh) = stack.remove(ci - 1) {
                stack.insert(ci - 1, AdvExpr::Value(lh + rh));
            }
        }
    }

    while stack.contains(&AdvExpr::Multiply) {
        let ci = stack.iter().position(|e| *e == AdvExpr::Multiply).unwrap();
        if let AdvExpr::Value(lh) = stack.remove(ci - 1) {
            stack.remove(ci - 1);
            if let AdvExpr::Value(rh) = stack.remove(ci - 1) {
                stack.insert(ci - 1, AdvExpr::Value(lh * rh));
            }
        }
    }

    assert_eq!(stack.len(), 1, "Did not fully reduce");
    return stack.pop().ok_or_else(|| EvalError);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_failure() {
        assert_eq!(eval_str("x + 2 * 3 + 4 * 5 + 6"), Err(EvalError));
    }

    #[test]
    fn test_get_next_op() {
        assert_eq!(get_next_op(" + 3").unwrap(), (Op::Add, "3"));
    }

    #[test]
    fn test_find_closing_paren() {
        assert_eq!(find_closing_paren("((a)bb())", 0), 8);
    }

    #[test]
    fn test_get_next_expr1() {
        assert_eq!(get_next_expr("1").unwrap(), (1, ""));
    }

    #[test]
    fn test_get_next_expr2() {
        assert_eq!(get_next_expr("1 + 2 + 3").unwrap(), (1, " + 2 + 3"));
    }

    #[test]
    fn test_get_next_expr3() {
        assert_eq!(get_next_expr("1 + (2 + 3)").unwrap(), (1, " + (2 + 3)"));
    }

    #[test]
    fn test_get_next_expr4() {
        assert_eq!(get_next_expr("(1 + 2) + 3").unwrap(), (3, " + 3"));
    }

    #[test]
    fn test_get_next_expr5() {
        assert_eq!(get_next_expr("(1 + 2)").unwrap(), (3, ""));
    }

    #[test]
    fn test1() {
        assert_eq!(eval_str("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 71);
    }

    #[test]
    fn test2() {
        assert_eq!(eval_str("2 * 3 + (4 * 5)").unwrap(), 26);
    }

    #[test]
    fn test3() {
        assert_eq!(eval_str("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), 437);
    }

    #[test]
    fn test4() {
        assert_eq!(
            eval_str("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            12240
        );
    }
    #[test]
    fn test5() {
        assert_eq!(
            eval_str("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            13632
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(eval_adv_expr("2 + (3 + 4)").unwrap(), AdvExpr::Value(9));
    }

    #[test]
    fn test1_adv() {
        assert_eq!(eval_adv("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }

    #[test]
    fn test2_adv() {
        assert_eq!(eval_adv("2 * 3 + (4 * 5)"), 46);
    }

    #[test]
    fn test3_adv() {
        assert_eq!(eval_adv("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            eval_adv("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            eval_adv("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
