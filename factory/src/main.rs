use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_machines(f: &str) -> std::io::Result<impl Iterator<Item = (u64, Vec<Vec<u8>>, Vec<u8>)>> {
    enum Token {
        Indicator(u64),
        Button(Vec<u8>),
        Joltage(Vec<u8>),
    }
    Ok(BufReader::new(File::open(f)?)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|l| {
            let (i, b, j) = l
                .split_whitespace()
                .filter_map(|t| {
                    let mut chars = t.chars();
                    chars.next_back(); // drop last char
                    match chars.next() {
                        Some(c) => match c {
                            '[' => Some(Token::Indicator(
                                chars
                                    .map(|c| c != '.')
                                    .enumerate()
                                    .map(|(i, b)| if b { 1 << (i as u64) } else { 0 })
                                    .sum(),
                            )),
                            '(' => Some(Token::Button(
                                chars
                                    .as_str()
                                    .split(',')
                                    .filter_map(|e| e.parse().ok())
                                    .collect(),
                            )),
                            '{' => Some(Token::Joltage(Vec::new())),
                            _ => None,
                        },
                        None => None,
                    }
                })
                .fold((None, Vec::new(), None), |(mut oi, mut vb, mut oj), t| {
                    match t {
                        Token::Indicator(i) => {
                            oi.replace(i);
                        }
                        Token::Button(b) => vb.push(b),
                        Token::Joltage(j) => {
                            oj.replace(j);
                        }
                    }
                    (oi, vb, oj)
                });
            i.and_then(move |i| j.map(move |j| (i, b, j)))
        }))
}

fn count_opt_buttons_pushes(wanted_indicators: u64, buttons: &[Vec<u8>]) -> u32 {
    fn solve_prog_dyn(state: u64, cache: &mut HashMap<(usize, u64), u32>, buttons: &[u64]) -> u32 {
        if state == 0 {
            return 0;
        }
        if buttons.is_empty() {
            return u32::MAX;
        };
        if let Some(res) = cache.get(&(buttons.len(), state)) {
            return *res;
        }
        let (first_button, remaining_buttons) = buttons.split_first().unwrap();
        // we try the only possibilities : pushing or not pushing button
        let not_pushing = solve_prog_dyn(state, cache, remaining_buttons);
        let pushing = {
            let new_state = state ^ *first_button;
            solve_prog_dyn(new_state, cache, remaining_buttons).saturating_add(1)
        };
        let res = not_pushing.min(pushing); // take the best solution
        cache.insert((buttons.len(), state), res);
        res
    }
    let mut cache = HashMap::new();
    let initial_state = wanted_indicators; // starting at wanted indicators and targetting 0
    // is same as starting at 0 and targetting wanted_indicators
    let buttons = buttons
        .iter()
        .map(|b| b.iter().map(|e| 1 << (*e as u64)).sum::<u64>())
        .collect::<Vec<u64>>();
    solve_prog_dyn(initial_state, &mut cache, &buttons)
}

fn main() -> std::io::Result<()> {
    println!(
        "opt buttons: {}",
        parse_machines("input")?
            .map(|(i, b, _)| count_opt_buttons_pushes(i, &b))
            .sum::<u32>()
    );
    Ok(())
}
