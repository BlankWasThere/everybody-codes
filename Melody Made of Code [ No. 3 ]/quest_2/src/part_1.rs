use std::collections::HashSet;

type Point = (i32, i32);

pub fn solve(input: &str) -> u32 {
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let end = parse_input(input);
    let start = (0, 0);

    let mut direction_idx = 0;
    let mut curr = start;
    let mut steps = 0;
    let mut visited = HashSet::from([start]);

    while curr != end {
        let (cx, cy) = curr;

        loop {
            let (dx, dy) = DIRECTIONS[direction_idx];
            direction_idx = (direction_idx + 1) % DIRECTIONS.len();

            let next = (cx + dx, cy + dy);
            if visited.insert(next) {
                curr = next;
                break;
            }
        }

        steps += 1;
    }

    steps
}

fn parse_input(input: &str) -> Point {
    let mut start = None;
    let mut end = None;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '@' => {
                    assert!(start.is_none());
                    start = Some((x as i32, y as i32));
                }
                '#' => {
                    assert!(end.is_none());
                    end = Some((x as i32, y as i32));
                }
                other => panic!("invalid character: {other}"),
            }
        }
    }

    let (sx, sy) = start.expect("should exist an starting point");
    let (ex, ey) = end.expect("should exist an ending point");

    (ex - sx, ey - sy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 12;
        let actual = solve(
            "\
.......
.......
.......
.#.@...
.......
.......
.......",
        );

        assert_eq!(expected, actual);
    }
}
