use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_range(r: &str) -> (u64, u64) {
    let mut numbers = r.split('-');
    let start = numbers
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or_default();
    let end = numbers
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or_default();
    (start, end)
}

// sort and remove intersections in ranges
fn fuse_ranges(mut raw_ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    raw_ranges.sort_unstable();
    raw_ranges
        .into_iter()
        .fold(Vec::new(), |mut v, (start2, end2)| {
            if let Some((_, end)) = v.last_mut()
                && start2 <= *end
            {
                *end = end2.max(*end);
            } else {
                v.push((start2, end2))
            }
            v
        })
}

fn parse_ingredients(f: &str) -> std::io::Result<(Vec<(u64, u64)>, impl Iterator<Item = u64>)> {
    let mut lines = BufReader::new(File::open(f)?).lines().map_while(|l| l.ok());
    // collect all ranges as they are
    let fresh_ingredients = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| parse_range(&l))
        .collect();
    // sort and remove intersections
    let fresh_ingredients = fuse_ranges(fresh_ingredients);
    Ok((fresh_ingredients, lines.filter_map(|l| l.parse().ok())))
}

fn main() -> std::io::Result<()> {
    let (fresh_ingredients, ingredients) = parse_ingredients("input")?;
    {
        println!(
            "number of fresh ingredients: {}",
            ingredients
                .filter(|ingredient| {
                    match fresh_ingredients.binary_search_by_key(ingredient, |(start, _)| *start) {
                        Ok(_) => true,
                        Err(i) => i
                            .checked_sub(1)
                            .map(|i| &fresh_ingredients[i])
                            .map(|(_, end)| end >= ingredient)
                            .unwrap_or_default(),
                    }
                })
                .count()
        );
    };
    let total_fresh = fresh_ingredients
        .iter()
        .map(|&(s, e)| e - s + 1)
        .sum::<u64>();
    println!("total fresh: {total_fresh}");

    Ok(())
}
