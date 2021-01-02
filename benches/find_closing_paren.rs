use criterion::{black_box, criterion_group, criterion_main, Criterion};

//use ::days::day18::find_closing_paren;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("find_closing_paren", |b| {
        b.iter(|| find_closing_paren("(()()()()(()()))", 0))
    });
    c.bench_function("find_closing_paren_iter", |b| {
        b.iter(|| find_closing_paren_iter("(()()()()(()()))", 0))
    });
}

// seven times slower womp womp
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

fn find_closing_paren_iter(s: &str, pi_lh: usize) -> usize {
    let mut pc = 0;
    for ci in pi_lh..s.len() {
        pc += match &s[ci..=ci] {
            "(" => 1,
            ")" => -1,
            _ => 0,
        };
        if pc == 0 {
            return ci + pi_lh;
        }
    }
    s.len()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
