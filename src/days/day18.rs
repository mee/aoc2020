use std::error::Error;
use std::str::FromStr;
pub fn day18() {
    let mut sum = 0;
    for line in include_str!("18.input").lines() {
        let res = eval_str(line).unwrap();
        println!("{:>12} = {}", res, line);
        sum += res;
    }
    println!("The sum of all results is {}", sum);
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
}
