use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use disjoint_set::DisjointSet; // let's not re-implement union-find ourselves
use itertools::{Either, Itertools};

fn parse_points(f: &str) -> std::io::Result<Vec<[i64; 3]>> {
    Ok(BufReader::new(File::open(f)?)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|l| l.split(',').filter_map(|e| e.parse().ok()).next_array())
        .collect())
}

fn compute_squared_distance(a: &[i64; 3], b: &[i64; 3]) -> i64 {
    a.iter()
        .zip(b)
        .map(|(ca, cb)| {
            let d = *ca - *cb;
            d * d
        })
        .sum()
}

fn min_cost_forest(
    points: &[[i64; 3]],
    wanted_connections: Option<usize>,
) -> Either<(usize, usize), impl Iterator<Item = usize>> {
    let mut trees = DisjointSet::new();
    for i in 0..points.len() {
        trees.make_set(i);
    }
    let mut segments = points
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i, pi), (j, pj))| {
            let distance = compute_squared_distance(pi, pj);
            (distance, i, j)
        })
        .collect::<Vec<_>>();
    segments.sort_unstable();
    let mut trees_number = points.len();
    let max_loops = wanted_connections.unwrap_or(segments.len());
    for (_, a, b) in segments.into_iter().take(max_loops) {
        if trees.find(a) != trees.find(b) {
            trees.union(a, b).unwrap();
            trees_number -= 1;
            if trees_number == 1 {
                return Either::Left((a, b));
            }
        }
    }
    let mut components_sizes: HashMap<_, usize> = HashMap::new();
    for dad in (0..points.len()).map(|me| trees.find(me).unwrap()) {
        *components_sizes.entry(dad).or_default() += 1;
    }
    Either::Right(components_sizes.into_values())
}

fn solve(points: &[[i64; 3]], limit: Option<usize>) {
    match min_cost_forest(points, limit) {
        Either::Left((a, b)) => {
            println!("product of x coordinates {}", points[a][0] * points[b][0])
        }
        Either::Right(sizes) => println!(
            "product of 3 largests: {}",
            sizes.k_largest(3).product::<usize>()
        ),
    }
}

fn main() -> std::io::Result<()> {
    let points = parse_points("input")?;
    solve(&points, Some(1000));
    solve(&points, None);
    Ok(())
}
