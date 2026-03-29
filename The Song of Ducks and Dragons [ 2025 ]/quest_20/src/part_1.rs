use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    let grid = parse_input(input);
    let init_position = (0usize, 0usize);

    let mut result = 0;
    let mut visited = HashSet::new();
    let mut stack = vec![(false, init_position)];

    while let Some((pointing_up, position @ (x, y))) = stack.pop() {
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
                && grid[ny][nx].is_some()
            {
                let next_position = (nx, ny);

                if grid[y][x] == Some(true)
                    && grid[ny][nx] == Some(true)
                    && !visited.contains(&next_position)
                {
                    result += 1;
                }

                stack.push((!pointing_up, next_position));
            }
        }
    }

    result
}

fn parse_input(input: &str) -> Vec<Vec<Option<bool>>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => None,
                    'T' => Some(true),
                    '#' => Some(false),
                    other => panic!("invalid character: `{other}`"),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 0;
        let actual = solve(
            "\
T#T#T#T#T#T
.T#T#T#T#T.
..T#T#T#T..
...T#T#T...
....T#T....
.....T.....",
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_2() {
        let expected = 0;
        let actual = solve(
            "\
T#T#T#T#T#T
.#T#T#T#T#.
..#T###T#..
...##T##...
....#T#....
.....#.....",
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_3() {
        let expected = 7;
        let actual = solve(
            "\
T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#.....",
        );

        assert_eq!(expected, actual);
    }
}
