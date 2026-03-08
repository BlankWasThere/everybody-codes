use std::collections::{HashSet, VecDeque};

use anyhow::{bail, ensure};

type Point = (i32, i32);

macro_rules! stable_min_max {
    ($a:expr, $b:expr) => {
        if $b > $a { ($a, $b) } else { ($b, $a) }
    };
}

const DIRECTIONS: [Point; 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

/// Returns the coordinates of walls and the destination point.\
/// The starting point is `(0, 0)`.
fn parse_input(input: &str) -> anyhow::Result<(Vec<Point>, Point)> {
    let mut curr = (0, 0);
    let mut walls = Vec::new();
    let mut direction_index: usize = 1;

    for (i, s) in input.trim().split(',').enumerate() {
        ensure!(s.len() >= 2, "invalid instruction: `{s}`");

        let (turn, steps) = s.split_at(1);
        let steps = steps.parse::<i32>()?;

        direction_index = match turn {
            "L" => direction_index.wrapping_sub(1).min(DIRECTIONS.len() - 1),
            "R" => (direction_index + 1) % DIRECTIONS.len(),
            other => bail!("invalid instruction: `{other}`"),
        };

        let direction = DIRECTIONS[direction_index];

        if i == 0 && steps != 1 {
            walls.push(add_points(curr, direction));
        }

        curr = add_points(curr, (direction.0 * steps, direction.1 * steps));
        walls.push(curr);
    }

    Ok((walls, curr))
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let (walls, dest) = parse_input(input)?;

    // Compress POI coordinates
    let (mut xs, mut ys): (Vec<_>, Vec<_>) = walls
        .iter()
        .copied()
        .map(|e| {
            [(-1, -1), (0, 0), (1, 1)]
                .into_iter()
                .map(move |diff| add_points(e, diff))
        })
        .flatten()
        .chain([(0, 0)])
        .unzip();

    xs.sort_unstable();
    xs.dedup();

    ys.sort_unstable();
    ys.dedup();

    let mut grid = vec![vec![false; xs.len()]; ys.len()];
    for (&(x1, y1), &(x2, y2)) in walls.iter().zip(walls.iter().skip(1)) {
        let cx1 = xs.binary_search(&x1).unwrap();
        let cy1 = ys.binary_search(&y1).unwrap();

        let cx2 = xs.binary_search(&x2).unwrap();
        let cy2 = ys.binary_search(&y2).unwrap();

        let (cx1, cx2) = stable_min_max!(cx1, cx2);
        let (cy1, cy2) = stable_min_max!(cy1, cy2);

        if cx1 == cx2 {
            for y in cy1..=cy2 {
                grid[y][cx1] = true;
            }
        } else if cy1 == cy2 {
            for x in cx1..=cx2 {
                grid[cy1][x] = true;
            }
        }
    }

    // // Uncomment this to visualize the compressed grid
    // for y in 0..grid.len() {
    //     for x in 0..grid[y].len() {
    //         if (xs[x], ys[y]) == (0, 0) {
    //             print!("S");
    //         } else if (xs[x], ys[y]) == dest {
    //             print!("E");
    //         } else if grid[y][x] {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let start = (
        xs.binary_search(&0).unwrap() as i32,
        ys.binary_search(&0).unwrap() as i32,
    );

    let dest = (
        xs.binary_search(&dest.0).unwrap() as i32,
        ys.binary_search(&dest.1).unwrap() as i32,
    );

    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);

    let total_steps = 'outer: loop {
        let (curr, steps) = queue.pop_front().expect("queue must not be empty");

        for direction in DIRECTIONS {
            let next_point = add_points(curr, direction);

            if next_point.0 < 0
                || next_point.0 >= grid[0].len() as i32
                || next_point.1 < 0
                || next_point.1 >= grid.len() as i32
            {
                continue;
            }

            let next_point_steps = steps + {
                let (rx1, ry1) = curr;
                let (rx2, ry2) = next_point;

                if rx1 == rx2 {
                    ys[ry2 as usize].abs_diff(ys[ry1 as usize])
                } else if ry1 == ry2 {
                    xs[rx2 as usize].abs_diff(xs[rx1 as usize])
                } else {
                    unreachable!()
                }
            };

            if next_point == dest {
                break 'outer next_point_steps;
            }

            let (nx, ny) = next_point;
            if grid[ny as usize][nx as usize] || !visited.insert(next_point) {
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
