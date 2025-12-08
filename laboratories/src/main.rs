use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_splitters(f: &str) -> std::io::Result<impl Iterator<Item = HashSet<usize>>> {
    Ok(BufReader::new(File::open(f)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| {
            l.char_indices()
                .filter_map(|(i, c)| (c == '^' || c == 'S').then_some(i))
                .collect()
        }))
}

fn question1() -> std::io::Result<()> {
    let mut splitters = parse_splitters("input")?;
    let start = splitters.next().unwrap().into_iter().next().unwrap();
    let mut splits = 0;
    splitters.fold(std::iter::once(start).collect::<HashSet<_>>(), |h, pos| {
        h.iter()
            .flat_map(|&p| {
                if pos.contains(&p) {
                    splits += 1;
                    [Some(p - 1), Some(p + 1)]
                } else {
                    [Some(p), None]
                }
            })
            .flatten()
            .collect()
    });
    println!("splits number: {}", splits);
    Ok(())
}

fn question2() -> std::io::Result<()> {
    let mut splitters = parse_splitters("input")?;
    let start = splitters.next().unwrap().into_iter().next().unwrap();
    let end_state = splitters.fold(
        std::iter::once((start, 1)).collect::<HashMap<_, _>>(),
        |h, pos| {
            h.iter().fold(HashMap::new(), |mut h, (&p, &c)| {
                if pos.contains(&p) {
                    *h.entry(p - 1).or_default() += c;
                    *h.entry(p + 1).or_default() += c;
                } else {
                    *h.entry(p).or_default() += c;
                }
                h
            })
        },
    );
    println!("paths number: {}", end_state.values().sum::<usize>());
    Ok(())
}

fn main() -> std::io::Result<()> {
    question1()?;
    question2()
}
