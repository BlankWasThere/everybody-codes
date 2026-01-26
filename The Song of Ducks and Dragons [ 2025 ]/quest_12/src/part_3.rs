use anyhow::anyhow;
use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);
type Grid = Vec<Vec<i8>>;
type Seen = HashSet<Point>;

fn parse_input(input: &str) -> anyhow::Result<Grid> {
    let mut grid = vec![];

    for line in input.lines().map(str::trim).filter(|s| !s.is_empty()) {
        let mut row = vec![];

        for c in line.chars() {
            row.push(
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("Invalid number: `{c}`"))? as _,
            );
        }

        grid.push(row);
    }

    Ok(grid)
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let grid = parse_input(input)?;

    let mut blown_barrels = vec![];
    {
        let mut seen = HashSet::new();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if seen.contains(&(r, c)) {
                    continue;
                }

                let barrels = solve_grid(&grid, &[(r, c)]);
                seen.extend(barrels.iter().copied());
                blown_barrels.push(barrels);
            }
        }
    }

    blown_barrels.sort_by_key(|v| v.len());
    let largest = blown_barrels.pop().unwrap();
    for s in &mut blown_barrels {
        s.retain(|i| !largest.contains(i));
    }

    blown_barrels.sort_by_key(|v| v.len());
    let second = blown_barrels.pop().unwrap();
    for s in &mut blown_barrels {
        s.retain(|i| !second.contains(i));
    }

    blown_barrels.sort_by_key(|v| v.len());
    let last = blown_barrels.pop().unwrap();

    let count = largest.len() + second.len() + last.len();
    println!("Answer: {count}");

    Ok(())
}

fn solve_grid(grid: &Grid, starting: &[Point]) -> Seen {
    let mut seen = Seen::new();

    let mut queue = starting.iter().copied().collect::<VecDeque<_>>();
    while let Some((i, j)) = queue.pop_front() {
        if !seen.insert((i, j)) {
            continue;
        }

        for (diff_i, diff_j) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let ni = i.checked_add_signed(diff_i);
            let nj = j.checked_add_signed(diff_j);

            if let (Some(ni), Some(nj)) = (ni, nj)
                && ni < grid.len()
                && nj < grid[ni].len()
                && grid[ni][nj] <= grid[i][j]
            {
                queue.push_back((ni, nj));
            }
        }
    }

    seen
}
