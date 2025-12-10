use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Bound, RangeInclusive};

type Point = [i64; 2];

fn parse_tiles(f: &str) -> std::io::Result<Vec<Point>> {
    Ok(BufReader::new(File::open(f)?)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|l| l.split(',').filter_map(|e| e.parse().ok()).next_array())
        .collect())
}

fn largest_rectangle_area(tiles: &[Point]) -> i64 {
    // we have 500 tiles, we can easily brute force
    tiles
        .iter()
        .tuple_combinations()
        .map(|([x1, y1], [x2, y2])| ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1))
        .max()
        .unwrap_or_default()
}

fn largest_green_red_area(tiles: &[[i64; 2]]) -> i64 {
    // let's burn some more carbon
    tiles
        .iter()
        .tuple_combinations() // loop on all squares
        .filter(|([x1, y1], [x2, y2])| {
            // check if the square is intersected by any segment
            tiles
                .iter()
                .circular_tuple_windows() // loop on all
                // segments
                .all(|([x3, y3], [x4, y4])| {
                    if x3 == x4 {
                        // vertical segment
                        x1 == x3
                            || x2 == x3
                            || ((x1 <= x3) == (x2 <= x3))
                            || (y1 <= y3.min(y4) && y2 <= y3.min(y4))
                            || (y1 >= y3.max(y4) && y2 >= y3.max(y4))
                    } else {
                        // horizontal segment
                        y1 == y3
                            || y2 == y3
                            || ((y1 <= y3) == (y2 <= y3))
                            || (x1 <= x3.min(x4) && x2 <= x3.min(x4))
                            || (x1 >= x3.max(x4) && x2 >= x3.max(x4))
                    }
                })
        })
        .map(|([x1, y1], [x2, y2])| ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1))
        .max()
        .unwrap_or_default()
}

fn main() -> std::io::Result<()> {
    let tiles = parse_tiles("input")?;
    println!("{}", largest_rectangle_area(&tiles));
    println!("{}", largest_green_red_area(&tiles));
    Ok(())
}
