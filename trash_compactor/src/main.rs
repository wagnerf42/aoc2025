use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Op {
    Add,
    Mul,
}

impl Op {
    fn neutral(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

fn parse_maths(f: &str) -> std::io::Result<(Vec<Vec<u64>>, Vec<Op>)> {
    let mut numbers = Vec::new();
    for line in BufReader::new(File::open(f)?).lines().map_while(|l| l.ok()) {
        if line
            .chars()
            .find(|c| !c.is_whitespace())
            .unwrap()
            .is_ascii_digit()
        {
            numbers.push(
                line.split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect(),
            )
        } else {
            return Ok((numbers, parse_ops(&line)));
        }
    }
    Result::Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "no ops",
    ))
}

// NOTE: not very pretty this time
// it would need some cleaning but well it's already been two hours
fn parse_and_compute_inverted_maths(f: &str) -> std::io::Result<u64> {
    let raw_file = std::fs::read_to_string(f)?;
    let lines: Vec<&str> = raw_file.lines().collect();
    let number_lines = &lines[..lines.len() - 1];
    let mut ops = parse_ops(lines.last().unwrap()).into_iter();
    let mut res = 0;
    let mut current_op = ops.next().unwrap();
    let mut column_res = current_op.neutral();
    for i in 0.. {
        if let Some(number) = number_lines
            .iter()
            .filter_map(|l| l.as_bytes().get(i).copied())
            .filter(|c| (48..=58).contains(c)) // meh, this is so bad
            .map(|c| c as u64 - 48)
            // but most ascii stuff
            // is in nightly
            .fold(None, |s, d| s.map(|s: u64| s * 10 + d).or(Some(d)))
        {
            match current_op {
                Op::Add => column_res += number,
                Op::Mul => column_res *= number,
            }
        } else {
            res += column_res;
            if let Some(op) = ops.next() {
                current_op = op;
                column_res = current_op.neutral();
            } else {
                return Ok(res);
            }
        }
    }
    Ok(res)
}

fn parse_ops(ops_line: &str) -> Vec<Op> {
    ops_line
        .split_whitespace()
        .map(|op| if op == "+" { Op::Add } else { Op::Mul })
        .collect()
}

// NOTE: i initially thought i could compute inverted columns
// from columns but it is not possible.
// i used a generic function which is not really needed anymore since this function
// only answers the first part.
// it has a nice lifetime issue though so i'll keep it.
fn compute_operations<'a, C, I>(numbers: &'a [Vec<u64>], ops: &[Op], mut compute_column: C) -> u64
where
    C: FnMut(&'a [Vec<u64>], usize) -> I, // NOTE: try removing the 'a here for some fun
    I: Iterator<Item = u64> + 'a,
{
    ops.iter()
        .enumerate()
        .map(|(i, o)| {
            let column = compute_column(numbers, i);
            match o {
                Op::Add => column.sum::<u64>(),
                Op::Mul => column.product(),
            }
        })
        .sum()
}

fn column(numbers: &[Vec<u64>], i: usize) -> impl Iterator<Item = u64> + '_ {
    numbers.iter().map(move |line| line[i])
}

fn main() -> std::io::Result<()> {
    let (numbers, ops) = parse_maths("input")?;
    let s: u64 = compute_operations(&numbers, &ops, column);
    println!("sum of operations : {s}");
    let s = parse_and_compute_inverted_maths("input")?;
    println!("sum of inverted operations : {s}");
    Ok(())
}
