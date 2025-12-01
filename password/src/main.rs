use std::{io::BufRead, ops::AddAssign};

fn parse_moves(f: &str) -> std::io::Result<impl Iterator<Item = i16>> {
    Ok(std::io::BufReader::new(std::fs::File::open(f)?)
        .lines()
        .map_while(|l| {
            l.ok().and_then(|l: String| {
                l.char_indices()
                    .next()
                    .and_then(|(i, c)| match c {
                        'R' => Some((i, 1)),
                        'L' => Some((i, -1)),
                        _ => None,
                    })
                    .and_then(|(i, s)| l[(i + 1)..].parse::<i16>().ok().map(|m| m * s))
            })
        }))
}

struct Position(u16);

impl AddAssign<i16> for Position {
    fn add_assign(&mut self, rhs: i16) {
        self.0 = ((((self.0 as i16 + rhs) % 100) + 100) % 100) as u16;
    }
}

impl Position {
    fn apply_move_and_count_zeroes(&mut self, movement: i16) -> u64 {
        let old_position_is_zero = self.0 == 0;
        let new_position = self.0 as i16 + movement;
        self.0 = ((new_position % 100 + 100) % 100) as u16;
        let clicks = (new_position / 100).abs()
            + if new_position == 0 { 1 } else { 0 }
            + if !old_position_is_zero && new_position < 0 {
                1
            } else {
                0
            };
        clicks as u64
    }
}

fn main() -> std::io::Result<()> {
    let zeroes = parse_moves("input")?
        .scan(Position(50), |p, m| {
            *p += m;
            Some(p.0)
        })
        .filter(|&p| p == 0)
        .count();

    println!("zeroes: {zeroes}");

    let zeroes_clicked: u64 = parse_moves("input")?
        .scan(Position(50), |p, m| Some(p.apply_move_and_count_zeroes(m)))
        .sum();

    println!("clicks: {zeroes_clicked}");

    Ok(())
}
