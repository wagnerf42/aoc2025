use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

struct Grid {
    places: Vec<Vec<bool>>,
}

impl Grid {
    fn new(f: &str) -> std::io::Result<Self> {
        Ok(Grid {
            places: BufReader::new(File::open(f)?)
                .lines()
                .map_while(|l| l.ok())
                .map(|l| l.chars().map(|c| c == '@').collect::<Vec<bool>>())
                .collect(),
        })
    }
    fn width(&self) -> usize {
        self.places[0].len()
    }
    fn height(&self) -> usize {
        self.places.len()
    }
    fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = bool> {
        let xmin = x.saturating_sub(1);
        let xmax = self.width().min(x + 2);
        let ymin = y.saturating_sub(1);
        let ymax = self.height().min(y + 2);
        (xmin..xmax)
            .cartesian_product(ymin..ymax)
            .filter(move |&(xn, yn)| xn != x || yn != y)
            .map(|(x, y)| self.places[y][x])
    }
    fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.width()).cartesian_product(0..self.height())
    }
    fn rolls_coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        self.coordinates().filter(|&(x, y)| self.places[y][x])
    }
    fn accessible_rolls(&self) -> impl Iterator<Item = (usize, usize)> {
        self.rolls_coordinates()
            .filter(|&(x, y)| self.neighbours(x, y).filter(|n| *n).count() < 4)
    }
    fn remove_accessibles(&mut self) -> usize {
        let to_remove = self.accessible_rolls().collect::<Vec<_>>(); // note : the
        // borrow checker is a pain here. removing them while computing them would be a different
        // algorithm but still ok.
        to_remove
            .iter()
            .for_each(|&(x, y)| self.places[y][x] = false);
        to_remove.len()
    }
}

fn main() -> std::io::Result<()> {
    let mut grid = Grid::new("input")?;
    let accessible = grid.accessible_rolls().count();
    println!("accessible {accessible}");
    let mut removed = 0;
    loop {
        let last_removed = grid.remove_accessibles();
        if last_removed == 0 {
            break;
        }
        removed += last_removed
    }
    println!("removed: {removed}");

    Ok(())
}
