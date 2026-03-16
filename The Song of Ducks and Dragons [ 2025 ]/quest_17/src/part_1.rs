use std::collections::{HashSet, VecDeque};

use anyhow::{Context, ensure};

type Point = (usize, usize);

pub fn solve(input: &str) -> anyhow::Result<u32> {
    const TARGET_RADIUS: f64 = 10.0;
    let (grid, starting_point) = parse_input(input)?;

    let mut sum = 0;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([starting_point]);
    while let Some(curr) = queue.pop_front() {
        if !visited.insert(curr) {
            continue;
        }

        let (x, y) = curr;
        sum += grid[y][x] as u32;

        for direction in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            if let (Some(x), Some(y)) = (
                curr.0.checked_add_signed(direction.0),
                curr.1.checked_add_signed(direction.1),
            ) && y < grid.len()
                && x < grid[y].len()
            {
                let next_point = (x, y);

                if distance(next_point, starting_point) <= TARGET_RADIUS {
                    queue.push_back(next_point);
                }
            }
        }
    }

    Ok(sum)
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vec<u8>>, Point)> {
    let mut grid = vec![];
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
        let expected = 1573;
        let actual = solve(
            "\
189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}
