// This solution uses bruteforce, and is slow. I may or may not fix this later...

use std::collections::HashSet;

type Point = (i32, i32);

const DIRECTIONS: [(i32, i32); 12] = [
    (0, -1), // U
    (0, -1),
    (0, -1),
    (1, 0), // R
    (1, 0),
    (1, 0),
    (0, 1), // D
    (0, 1),
    (0, 1),
    (-1, 0), // L
    (-1, 0),
    (-1, 0),
];

pub fn solve(input: &str) -> u32 {
    let destinations = parse_input(input);
    let start = (0, 0);

    // Since the inner points can be surrounded if the outer (boundary) points are, we can remove them.
    let mut remaining = destinations
        .iter()
        .copied()
        .filter(|&p| {
            !DIRECTIONS
                .into_iter()
                .all(|d| destinations.contains(&add_points(p, d)))
        })
        .collect::<Vec<_>>();

    let mut surrounded_cache = HashSet::new();
    let mut visited: HashSet<(i32, i32)> = remaining
        .iter()
        .copied()
        .chain([start])
        .collect::<HashSet<_>>();

    let mut steps = 0;
    let mut curr = start;

    for direction in DIRECTIONS.into_iter().cycle() {
        if remaining.is_empty() {
            break;
        }

        let next = add_points(curr, direction);
        if visited.insert(next) && !surrounded(next, &visited, &mut surrounded_cache) {
            curr = next;
            steps += 1;
            remaining.retain(|&p| !surrounded(p, &visited, &mut surrounded_cache));
        }
    }

    steps
}

fn surrounded(point: Point, visited: &HashSet<Point>, cache: &mut HashSet<Point>) -> bool {
    fn inner(
        point: Point,
        visited: &HashSet<Point>,
        cache: &HashSet<Point>,
        checked_points: &mut HashSet<Point>,
        (min, max): (Point, Point),
    ) -> bool {
        let (min_x, min_y) = min;
        let (max_x, max_y) = max;

        // Extend a ray to every direction and see if there exist a visited point
        if !DIRECTIONS.into_iter().all(|direction| match direction {
            (0, _) => {
                let (px, mut py) = add_points(point, direction);
                while py <= max_y && py >= min_y {
                    if visited.contains(&(px, py)) {
                        return true;
                    }

                    (_, py) = add_points((px, py), direction);
                }

                false
            }
            (_, 0) => {
                let (mut px, py) = add_points(point, direction);
                while px <= max_x && px >= min_x {
                    if visited.contains(&(px, py)) {
                        return true;
                    }

                    (px, _) = add_points((px, py), direction);
                }

                false
            }
            _ => unreachable!(),
        }) {
            return false;
        }

        let next_points = DIRECTIONS
            .into_iter()
            .map(|direction| add_points(point, direction))
            .filter(|&p| !visited.contains(&p) && !cache.contains(&p) && checked_points.insert(p))
            .collect::<Vec<_>>();

        next_points.into_iter().all(|p| {
            inner(
                p,
                visited,
                cache,
                checked_points,
                ((min_x, min_y), (max_x, max_y)),
            )
        })
    }

    if visited.is_empty() {
        return false;
    }

    if cache.contains(&point) {
        return true;
    }

    // Find grid boundaries
    let mut max_x = None;
    let mut max_y = None;
    let mut min_x = None;
    let mut min_y = None;

    for &(px, py) in visited {
        if max_x.is_none_or(|max| px > max) {
            max_x = Some(px);
        }

        if max_y.is_none_or(|max| py > max) {
            max_y = Some(py);
        }

        if min_x.is_none_or(|min| px < min) {
            min_x = Some(px);
        }

        if min_y.is_none_or(|min| py < min) {
            min_y = Some(py);
        }
    }

    let max_x = max_x.unwrap();
    let max_y = max_y.unwrap();
    let min_x = min_x.unwrap();
    let min_y = min_y.unwrap();

    let mut checked_points = HashSet::from([point]);

    let result = inner(
        point,
        visited,
        cache,
        &mut checked_points,
        ((min_x, min_y), (max_x, max_y)),
    );

    if result {
        cache.extend(checked_points);
    }

    result
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
