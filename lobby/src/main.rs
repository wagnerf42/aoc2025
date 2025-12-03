use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_banks(f: &str) -> std::io::Result<impl Iterator<Item = Vec<u8>>> {
    Ok(BufReader::new(File::open(f)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect()
        }))
}

fn find_position_of_max_and_max(bank: &[u8]) -> (usize, u8) {
    bank.iter().enumerate().fold(
        (0, 0),
        |(pos_max, max), (pos_e, &e)| {
            if e > max { (pos_e, e) } else { (pos_max, max) }
        },
    )
}

fn best_joltage(bank: &[u8]) -> u8 {
    let (pos_max, max) = find_position_of_max_and_max(&bank[..bank.len() - 1]);
    max * 10 + bank[(pos_max + 1)..].iter().max().unwrap()
}

fn best_joltage12(bank: &[u8]) -> u64 {
    (0..12)
        .rev()
        .fold((0u64, bank), |(joltage, remaining), wanted_digits| {
            let (pos_max, max) =
                find_position_of_max_and_max(&remaining[..remaining.len() - wanted_digits]);
            (10 * joltage + max as u64, &remaining[(pos_max + 1)..])
        })
        .0
}

fn main() -> std::io::Result<()> {
    let s: u64 = read_banks("input")?.map(|b| best_joltage(&b) as u64).sum();
    println!("{s}");
    let s: u64 = read_banks("input")?.map(|b| best_joltage12(&b)).sum();
    println!("{s}");
    Ok(())
}
