use itertools::Itertools;
use std::{cmp::max, io::BufRead};

fn parse_ranges(line: &str) -> impl Iterator<Item = std::ops::RangeInclusive<u64>> {
    line.split(",").filter_map(|range_str| {
        let mut limits = range_str
            .split("-")
            .filter_map(|int_str| int_str.parse::<u64>().ok());
        let start = limits.next();
        let end = limits.next();
        start.and_then(|s| end.map(|e| s..=e))
    })
}

fn digits(num: u64) -> Vec<u8> {
    std::iter::successors(Some(num), |&n| if n < 10 { None } else { Some(n / 10) })
        .map(|n| (n % 10) as u8)
        .collect()
}

fn invalid_digits(num: u64) -> bool {
    let digits = digits(num);
    let mid = digits.len() / 2;
    let (weak, strong) = digits.split_at(mid);
    weak == strong
}

fn invalid_digits2(num: u64) -> bool {
    if num < 10 {
        return false;
    }
    let digits = digits(num);
    let max_block_size = digits.len() / 2;
    (1..=max_block_size).any(|size| {
        if !digits.len().is_multiple_of(size) {
            return false;
        }
        digits.chunks_exact(size).all_equal()
    })
}

fn main() {
    let ranges_line = std::fs::read_to_string("input").unwrap();
    let sum = parse_ranges(ranges_line.trim_end())
        .flatten()
        .filter(|&num| invalid_digits(num))
        .sum::<u64>();
    println!("{sum}");

    let sum = parse_ranges(ranges_line.trim_end())
        .flatten()
        .filter(|&num| invalid_digits2(num))
        .sum::<u64>();
    println!("{sum}");
}
