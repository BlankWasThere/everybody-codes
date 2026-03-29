use std::{
    collections::{HashSet, VecDeque},
    iter,
};

type Trampolines = Vec<Vec<bool>>;
type Point = (usize, usize);

pub fn solve(input: &str) -> u32 {
    let (grid, start, end) = parse_input(input);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(false, start, 0)]);

    while let Some((pointing_up, position, steps)) = queue.pop_front() {
        if position == end {
            return steps;
        }

        if !visited.insert(position) {
            continue;
        }

        let rotated_position = rotate_pos(&grid, position);

        for next_position in next_positions(&grid, rotated_position) {
            queue.push_back((!pointing_up, next_position, steps + 1));
        }
    }

    unreachable!()
}

fn rotate_pos(grid: &Trampolines, pos: Point) -> Point {
    let (col, row) = pos;

    let next_row = (grid[row].len() - 1 - col) / 2;
    let next_col = grid[next_row].len() - 1 - col;

    (next_col, next_row)
}

fn next_positions(grid: &Trampolines, curr: Point) -> impl Iterator<Item = Point> {
    let mut neighbours = vec![];
    let (col, row) = curr;

    if col > 0 {
        neighbours.push((col - 1, row));
    }

    if col < grid[row].len() - 1 {
        neighbours.push((col + 1, row));
    }

    if col % 2 == 0 {
        if row > 0 {
            neighbours.push((col + 1, row - 1));
        }
    } else {
        neighbours.push((col - 1, row + 1));
    }

    neighbours
        .into_iter()
        .chain(iter::once(curr))
        .filter(|&(col, row)| grid[row][col])
}

fn parse_input(input: &str) -> (Trampolines, Point, Point) {
    let mut start = None;
    let mut end = None;

    let trampolines = input
        .lines()
        .map(|l| l.trim().trim_matches('.'))
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    'S' => {
                        start = Some((col, row));
                        true
                    }
                    'E' => {
                        end = Some((col, row));
                        true
                    }
                    'T' => true,
                    '#' => false,
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
        let expected = 23;
        let actual = solve(
            "\
T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S.........",
        );

        assert_eq!(expected, actual);
    }
}
