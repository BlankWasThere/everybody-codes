use std::collections::{HashSet, VecDeque};

use anyhow::{bail, ensure};

type Point = (i32, i32);
type Walls = HashSet<Point>;

const DIRECTIONS: [Point; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// Returns the coordinates of walls and the destination point.\
/// The starting point is `(0, 0)`.
fn parse_input(input: &str) -> anyhow::Result<(Walls, Point)> {
    let mut curr = (0, 0);
    let mut walls = Walls::from([curr]);
    let mut direction_index: usize = 0;

    for s in input.trim().split(',') {
        ensure!(s.len() >= 2, "invalid instruction: `{s}`");

        let (turn, steps) = s.split_at(1);
        let steps = steps.parse::<u8>()?;

        direction_index = match turn {
            "L" => direction_index.wrapping_sub(1).min(DIRECTIONS.len() - 1),
            "R" => (direction_index + 1) % DIRECTIONS.len(),
            other => bail!("invalid instruction: `{other}`"),
        };

        let direction = DIRECTIONS[direction_index];

        walls.extend((0..steps).map(|_| {
            curr = add_points(curr, direction);
            curr
        }));
    }

    Ok((walls, curr))
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let (walls, dest) = parse_input(input)?;

    // visualize_maze(&walls, (0, 0), dest);

    let start = (0, 0);
    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);

    let (bound_min, bound_max) = {
        let x_min = walls.iter().map(|e| &e.0).copied().min().unwrap();
        let x_max = walls.iter().map(|e| &e.0).copied().max().unwrap();
        let y_min = walls.iter().map(|e| &e.1).copied().min().unwrap();
        let y_max = walls.iter().map(|e| &e.1).copied().max().unwrap();
        (
            add_points((x_min, y_min), (-1, -1)),
            add_points((x_max, y_max), (1, 1)),
        )
    };

    let total_steps = 'outer: loop {
        let (curr, steps) = queue.pop_front().expect("queue must not be empty");

        for direction in DIRECTIONS {
            let next_point = add_points(curr, direction);
            let next_point_steps = steps + 1;

            if next_point == dest {
                break 'outer next_point_steps;
            }

            if walls.contains(&next_point)
                || !visited.insert(next_point)
                || next_point < bound_min
                || next_point > bound_max
            {
                continue;
            }

            queue.push_back((next_point, next_point_steps));
        }
    };

    println!("Answer: {total_steps}");
    Ok(())
}

fn add_points(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

#[allow(unused)]
fn visualize_maze(walls: &Walls, start: Point, end: Point) {
    let (bound_min, bound_max) = {
        let x_min = walls.iter().map(|e| &e.0).copied().min().unwrap();
        let x_max = walls.iter().map(|e| &e.0).copied().max().unwrap();
        let y_min = walls.iter().map(|e| &e.1).copied().min().unwrap();
        let y_max = walls.iter().map(|e| &e.1).copied().max().unwrap();
        (
            add_points((x_min, y_min), (-1, -1)),
            add_points((x_max, y_max), (1, 1)),
        )
    };

    for y in bound_min.1..=bound_max.1 {
        for x in bound_min.0..=bound_max.0 {
            if (x, y) == start {
                print!("S");
            } else if (x, y) == end {
                print!("E");
            } else if walls.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}
