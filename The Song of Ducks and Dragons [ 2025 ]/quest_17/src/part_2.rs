use std::collections::{HashSet, VecDeque};

use anyhow::{Context, ensure};

type Point = (usize, usize);

pub fn solve(input: &str) -> anyhow::Result<u32> {
    let (grid, starting_point) = parse_input(input)?;

    let mut visited = HashSet::from([starting_point]);
    let mut queue = VecDeque::from([starting_point]);
    let mut radius = 0.0;
    let mut greatest = 0;
    let mut greatest_step = 0;

    for step in 0.. {
        let mut sum: u32 = 0;
        let mut is_last = false;
        let mut remaining = queue.len();

        while remaining > 0
            && let Some(curr) = queue.pop_front()
        {
            remaining -= 1;

            let (x, y) = curr;
            sum += grid[y][x] as u32;

            if [0, grid.len() - 1].contains(&curr.1) || [0, grid[0].len() - 1].contains(&curr.0) {
                is_last = true;
            }

            for direction in [
                (0, -1),
                (-1, 0),
                (0, 1),
                (1, 0),
                (-1, -1),
                (1, -1),
                (-1, 1),
                (1, 1),
            ] {
                if let (Some(x), Some(y)) = (
                    curr.0.checked_add_signed(direction.0),
                    curr.1.checked_add_signed(direction.1),
                ) && y < grid.len()
                    && x < grid[y].len()
                    && distance((x, y), starting_point) <= radius + 1.0
                    && visited.insert((x, y))
                {
                    queue.push_back((x, y));
                }
            }
        }

        if sum >= greatest {
            greatest_step = step;
            greatest = sum;
        }

        if is_last {
            break;
        }

        radius += 1.0;
    }

    Ok(greatest * greatest_step)
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vec<u8>>, Point)> {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut starting_pos = None;

    for (i, line) in input.trim().lines().enumerate() {
        let mut row = vec![];

        for c in line.chars() {
            if c == '@' {
                ensure!(starting_pos.is_none());
                starting_pos = Some((i, row.len()));
                row.push(0);
            } else {
                row.push(c.to_digit(10).context(format!("invalid interger: `{c}`"))? as u8);
            }
        }

        if let Some(first) = grid.first() {
            ensure!(row.len() == first.len());
        }

        grid.push(row);
    }

    let starting_pos = starting_pos.context("starting position not in input")?;

    Ok((grid, starting_pos))
}

fn distance(a: Point, b: Point) -> f64 {
    let xdiff = a.0.abs_diff(b.0);
    let ydiff = a.1.abs_diff(b.1);

    ((xdiff * xdiff + ydiff * ydiff) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 1090;
        let actual = solve(
            "\
4547488458944
9786999467759
6969499575989
7775645848998
6659696497857
5569777444746
968586@767979
6476956899989
5659745697598
6874989897744
6479994574886
6694118785585
9568991647449",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}
