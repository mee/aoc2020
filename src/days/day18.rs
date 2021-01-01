use std::error::Error;
use std::str::FromStr;
pub fn day18() {}

#[derive(Debug)]
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

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

impl Op {
    fn apply(self: &Self, a: isize, b: isize) -> isize {
        eprintln!("Performing {:?} on {}, {}", self, a, b);
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
        }
    }
}

impl FromStr for Op {
    type Err = OpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "*" => Op::Multiply,
            "+" => Op::Add,
            _ => panic!("ahhhhh! '{}'", s),
        })
    }
}

fn eval_str(s: &str) -> Result<isize, EvalError> {
    // holy "unwrap", batman
    if let Some(pi_lh) = s.find('(') {
        // parse expr + op on lhs of (, op + expr on rhs of ) and return

        // does rust have scanl?
        let pi_rh = {
            let mut pc: isize = 1;
            let mut ci = pi_lh + 1;
            loop {
                let c = &s[ci..ci + 1];
                pc += match c {
                    "(" => 1,
                    ")" => -1,
                    _ => 0,
                };
                if pc == 0 {
                    break ci;
                } else if ci == s.len() - 1 {
                    panic!("no closing paren found");
                }
                ci += 1;
            }
        };

        dbg!(s, pi_lh, pi_rh);

        // is there an lhs expression? lhs op ( in ) ?
        if pi_lh > 0 {
            // is there a rhs expression?
            if pi_rh < s.len() - 1 {
                // lhs op ( in ) op rhs
                let expr_lh = &s[0..pi_lh - 3];
                let expr_in = &s[pi_lh + 1..pi_rh];
                let expr_rh = &s[pi_rh + 4..];
                let op_lh = &s[pi_lh - 2..pi_lh - 1]
                    .parse::<Op>()
                    .map_err(|_| EvalError)?;
                let op_rh = &s[pi_rh + 2..pi_rh + 3]
                    .parse::<Op>()
                    .map_err(|_| EvalError)?;

                return Ok(op_rh.apply(
                    op_lh.apply(eval_str(expr_lh)?, eval_str(expr_in)?),
                    eval_str(expr_rh)?,
                ));
            } else {
                // lhs op ( in )
                let expr_lh = &s[0..pi_lh - 3];
                let expr_in = &s[pi_lh + 1..pi_rh];
                let op_lh = &s[pi_lh - 2..pi_lh - 1]
                    .parse::<Op>()
                    .map_err(|_| EvalError)?;

                return Ok(op_lh.apply(eval_str(expr_lh)?, eval_str(expr_in)?));
            }
        } else {
            // is there an rhs expression?
            if pi_rh < s.len() - 1 {
                // ( in ) op_rh rhs
                let expr_in = &s[pi_lh + 1..pi_rh];
                let expr_rh = &s[pi_rh + 4..];
                let op_rh = &s[pi_rh + 2..pi_rh + 3]
                    .parse::<Op>()
                    .map_err(|_| EvalError)?;

                return Ok(op_rh.apply(eval_str(expr_in)?, eval_str(expr_rh)?));
            } else {
                // ( in )
                let expr_in = &s[pi_lh + 1..pi_rh];
                return Ok(eval_str(expr_in)?);
            }
        }
    } else {
        let mut parts = s.split(' ').collect::<Vec<&str>>();
        println!("Parsing final reduced expression: {:?}", &parts);
        parts.reverse();
        let mut res: isize = parts
            .pop()
            .ok_or_else(|| EvalError)?
            .parse::<isize>()
            .map_err(|_| EvalError)?;

        while parts.len() >= 2 {
            let op = parts
                .pop()
                .ok_or_else(|| EvalError)?
                .parse::<Op>()
                .map_err(|_| EvalError)?;
            let rhs = parts
                .pop()
                .ok_or_else(|| EvalError)?
                .parse::<isize>()
                .map_err(|_| EvalError)?;
            res = op.apply(res, rhs);
        }
        return Ok(res);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_failure() {
        assert_eq!(eval_str("x + 2 * 3 + 4 * 5 + 6").unwrap(), 71);
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
