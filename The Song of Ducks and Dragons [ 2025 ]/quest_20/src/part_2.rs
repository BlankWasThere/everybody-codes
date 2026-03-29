use std::collections::{HashSet, VecDeque};

type Trampolines = Vec<Vec<Option<bool>>>;
type Point = (usize, usize);

pub fn solve(input: &str) -> u32 {
    let (grid, start, end) = parse_input(input);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(false, start, 0)]);

    while let Some((pointing_up, position @ (x, y), steps)) = queue.pop_front() {
        if position == end {
            return steps;
        }

        if !visited.insert(position) {
            continue;
        }

        for direction @ (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            if (pointing_up && direction == (0, -1)) || (!pointing_up && direction == (0, 1)) {
                continue;
            }

            if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
                && ny < grid.len()
                && nx < grid[ny].len()
                && grid[ny][nx] == Some(true)
            {
                let next_position = (nx, ny);
                queue.push_back((!pointing_up, next_position, steps + 1));
            }
        }
    }

    unreachable!()
}

fn parse_input(input: &str) -> (Trampolines, Point, Point) {
    let mut start = None;
    let mut end = None;

    let trampolines = input
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => None,
                    'S' => {
                        start = Some((col, row));
                        Some(true)
                    }
                    'E' => {
                        end = Some((col, row));
                        Some(true)
                    }
                    'T' => Some(true),
                    '#' => Some(false),
                    other => panic!("invalid character: `{other}`"),
                })
                .collect()
        })
        .collect();

    (trampolines, start.unwrap(), end.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 32;
        let actual = solve(
            "\
TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........",
        );

        assert_eq!(expected, actual);
    }
}
