use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use anyhow::{bail, ensure};

type Point = (i32, i32);
type Walls = Vec<Point>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridCell {
    /// Coordinates of current cell.
    point: Point,
    /// Number of steps from start -> current.
    steps: u32,
    /// Cost from start -> current.
    g: u32,
    /// Cost from current -> end.
    h: u32,
}

impl Ord for GridCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f().cmp(&other.f())
    }
}

impl PartialOrd for GridCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl GridCell {
    pub fn new(point: Point, dest: Point, parent: Option<Self>) -> Self {
        let mut cell = GridCell {
            point,
            steps: 0,
            g: 0,
            h: calculate_distance(point, dest),
        };

        if let Some(parent) = parent {
            cell.steps = parent.steps + 1;
            cell.g = parent.g + calculate_distance(point, parent.point);
            cell.h = calculate_distance(point, dest);
        }

        cell
    }

    pub fn f(&self) -> u32 {
        self.g + self.h
    }
}

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

    let start = GridCell::new((0, 0), dest, None);
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::from([Reverse(start)]);

    let total_steps = 'outer: loop {
        let curr = queue
            .pop()
            .expect("heap should not be empty during A* search")
            .0;

        for direction in DIRECTIONS {
            let next_point = add_points(curr.point, direction);
            let next_cell = GridCell::new(next_point, dest, Some(curr));
            let next_cell_cost = next_cell.f();

            if next_point == dest {
                break 'outer next_cell.steps;
            }

            if walls.contains(&next_point) {
                continue;
            }

            if let Some(Reverse(other)) = queue.iter().find(|e| e.0.point == next_point)
                && other.f() <= next_cell_cost
            {
                continue;
            }

            if let Some(other_f) = visited.get_mut(&next_point) {
                if *other_f <= next_cell_cost {
                    continue;
                } else {
                    *other_f = next_cell_cost;
                }
            }

            queue.push(Reverse(next_cell));
        }

        visited.insert(curr.point, curr.f());
    };

    println!("Answer: {total_steps}");
    Ok(())
}

fn calculate_distance(p1: Point, p2: Point) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn add_points(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}
