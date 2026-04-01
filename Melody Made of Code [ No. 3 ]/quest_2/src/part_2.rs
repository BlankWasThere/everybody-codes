use std::collections::HashSet;

type Point = (i32, i32);

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve(input: &str) -> u32 {
    let end = parse_input(input);
    let start = (0, 0);

    let mut direction_idx = 0;
    let mut curr = start;
    let mut steps = 0;
    let mut visited = HashSet::from([start, end]);

    while !surrounded(end, &visited) {
        loop {
            let direction = DIRECTIONS[direction_idx];
            direction_idx = (direction_idx + 1) % DIRECTIONS.len();

            let next = add_points(curr, direction);
            if visited.insert(next) && !surrounded(next, &visited) {
                curr = next;
                break;
            }
        }

        steps += 1;
    }

    steps
}

fn surrounded(point: Point, visited: &HashSet<Point>) -> bool {
    fn inner(
        point: Point,
        visited: &HashSet<Point>,
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
            .filter(|&p| !visited.contains(&p) && checked_points.insert(p))
            .collect::<Vec<_>>();

        next_points
            .into_iter()
            .all(|p| inner(p, visited, checked_points, ((min_x, min_y), (max_x, max_y))))
    }

    if visited.is_empty() {
        return false;
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

    inner(
        point,
        visited,
        &mut HashSet::from([point]),
        ((min_x, min_y), (max_x, max_y)),
    )
}

fn add_points((px, py): Point, (dx, dy): Point) -> Point {
    (px + dx, py + dy)
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
        let expected = 47;
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
