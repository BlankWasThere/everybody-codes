use std::collections::{HashSet, VecDeque};

type Point = (i32, i32);

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[rustfmt::skip]
const MOVES: [(i32, i32); 12] = [
    DIRECTIONS[0],
    DIRECTIONS[0],
    DIRECTIONS[0],

    DIRECTIONS[1],
    DIRECTIONS[1],
    DIRECTIONS[1],

    DIRECTIONS[2],
    DIRECTIONS[2],
    DIRECTIONS[2],

    DIRECTIONS[3],
    DIRECTIONS[3],
    DIRECTIONS[3],
];

pub fn solve(input: &str) -> u32 {
    let targets = parse_input(input);
    let start = (0, 0);

    let mut steps = 0;
    let mut curr = start;
    let mut visited = HashSet::from_iter(targets.iter().copied().chain([start]));

    // Remove inner points, as enclosing outer points will automatically enclose inner points
    let mut remaining = targets
        .into_iter()
        .filter(|&p| {
            surround(p, &mut visited);

            !DIRECTIONS
                .into_iter()
                .all(|d| visited.contains(&add_points(p, d)))
        })
        .collect::<HashSet<_>>();

    for r#move in MOVES.into_iter().cycle() {
        if remaining.is_empty() {
            break;
        }

        let next_point = add_points(curr, r#move);

        if !visited.insert(next_point) {
            continue;
        }

        steps += 1;
        curr = next_point;
        surround(curr, &mut visited);

        remaining.retain(|&p| {
            !DIRECTIONS
                .into_iter()
                .all(|d| visited.contains(&add_points(p, d)))
        });
    }

    steps
}

fn surround(point: Point, visited: &mut HashSet<Point>) {
    // Find grid boundaries
    let mut max_x = None;
    let mut max_y = None;
    let mut min_x = None;
    let mut min_y = None;

    for &(px, py) in visited.iter() {
        if max_x.is_none_or(|max| px > max) {
            max_x = Some(px + 1);
        }

        if max_y.is_none_or(|max| py > max) {
            max_y = Some(py + 1);
        }

        if min_x.is_none_or(|min| px < min) {
            min_x = Some(px - 1);
        }

        if min_y.is_none_or(|min| py < min) {
            min_y = Some(py - 1);
        }
    }

    let max_x = max_x.unwrap();
    let max_y = max_y.unwrap();
    let min_x = min_x.unwrap();
    let min_y = min_y.unwrap();

    // Do flood fill in all directions and stop if it overflows the boundary
    'outer: for direction in DIRECTIONS {
        let starting_point = add_points(point, direction);
        let mut ff_visited = HashSet::new();
        let mut queue = VecDeque::from([starting_point]);

        while let Some(curr @ (cx, cy)) = queue.pop_front() {
            if cx > max_x || cx < min_x || cy > max_y || cy < min_y {
                continue 'outer;
            }

            if visited.contains(&curr) || !ff_visited.insert(curr) {
                continue;
            }

            queue.extend(DIRECTIONS.into_iter().map(|d| add_points(curr, d)))
        }

        visited.extend(ff_visited);
    }
}

fn add_points((px, py): Point, (dx, dy): Point) -> Point {
    (px + dx, py + dy)
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut start = None;
    let mut destinations = vec![];
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '@' => {
                    assert!(start.is_none());
                    start = Some((x as i32, y as i32));
                }
                '#' => {
                    destinations.push((x as i32, y as i32));
                }
                other => panic!("invalid character: {other}"),
            }
        }
    }

    let (sx, sy) = start.expect("should exist an starting point");

    destinations
        .into_iter()
        .map(|(ex, ey)| (ex - sx, ey - sy))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 87;
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

    #[test]
    fn test_2() {
        let expected = 239;
        let actual = solve(
            "\
#..#.......#...
...#...........
...#...........
#######........
...#....#######
...#...@...#...
...#.......#...
...........#...
...........#...
#..........#...
##......#######",
        );

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_3() {
        let expected = 1539;
        let actual = solve(
            "\
................................................................
.........................###.........###........................
....................##...###########.#####......#.......###.....
.........##.............############....####.............##.....
.......######..............#############.###....................
.........##................#############.###.......##...........
...............##...........########....####....................
...............................####.#######...........##........
........................##################...........####.......
....#.........#########################.....##......######......
..............#.##......##....##..##.##...............##........
..............................##....##..........##..............
........####....#################..######...................##..
........###.....###...####..###..##...##.########...............
.................####....###..##.##.##..###....##.....##........
....##...........#######.....##..##..##......#####..........#...
...........##......#########......#....##.######..........#####.
...........##........###########################....#.......#...
.........######............##################.......#...........
...........##.............#########.............................
............#.........#############....................#........
.....#...........##..####......###......##........#.............
.............##................###..........#.....#.............
..................##...........##...................##..........
..........................###.####.####.........................
................#.###########..###.############.#...............
.....#####....###...............................###.............
.....#####...#############......@......#############............
.....#########.###################################.#............
...###########..##.....###################.....##..##...........
...######...#######.##...###.........##...##...###.##...........
.....##.########........#####..###..####.......#.########.......
............#########################################...........
..............#####################################.............
...............................###..............................
................................................................",
        );

        assert_eq!(expected, actual);
    }
}
