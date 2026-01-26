use std::collections::VecDeque;

use anyhow::anyhow;

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i8>>> {
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
    let mut grid = parse_input(input)?;
    let mut count = 0;

    let mut queue = VecDeque::from([(0, 0), (grid.len() - 1, grid[0].len() - 1)]);
    while let Some((i, j)) = queue.pop_front() {
        let curr = grid[i][j];
        if curr == -1 {
            continue;
        }

        count += 1;
        grid[i][j] = -1;

        for (diff_i, diff_j) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let i = i.wrapping_add_signed(diff_i);
            let j = j.wrapping_add_signed(diff_j);

            if i < grid.len() && j < grid[i].len() && grid[i][j] != -1 && grid[i][j] <= curr {
                queue.push_back((i, j));
            }
        }
    }

    println!("Answer: {count}");

    Ok(())
}
