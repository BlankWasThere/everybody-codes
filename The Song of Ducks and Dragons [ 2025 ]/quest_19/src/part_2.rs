use std::collections::{BTreeSet, HashSet};

pub fn solve(input: &str) -> u32 {
    let input = parse_input(input);
    let start = (0, 0);
    let dest_x = input.last().unwrap()[0];
    let passages = input
        .iter()
        .copied()
        .flat_map(|[x, y, height]| {
            let x = start.1 + x;
            ((start.1 + y)..(start.1 + y + height)).map(move |y| (x, y))
        })
        .collect::<HashSet<_>>();

    let mut walls_x = HashSet::new();
    let y_max = passages
        .iter()
        .map(|&(x, y)| {
            walls_x.insert(x);
            y
        })
        .max()
        .unwrap();

    let mut min_flaps: Option<u32> = None;
    let mut queue = BTreeSet::from([(0, (0, 0))]);
    while let Some((flaps, position @ (x, y))) = queue.pop_first() {
        if walls_x.contains(&x) && !passages.contains(&position) {
            continue;
        }

        if position.0 == dest_x {
            min_flaps = Some(if let Some(min_flaps) = min_flaps {
                min_flaps.min(flaps)
            } else {
                flaps
            });

            continue;
        }

        for (flap_delta, dy) in [(1, 1), (0, -1)] {
            if let Some(ny) = y.checked_add_signed(dy)
                && ny <= y_max
            {
                queue.insert((flaps + flap_delta, (x + 1, ny)));
            }
        }
    }

    min_flaps.unwrap()
}

fn parse_input(input: &str) -> Vec<[u32; 3]> {
    input
        .trim()
        .lines()
        .map(|s| {
            s.split(',')
                .map(|c| {
                    c.parse()
                        .unwrap_or_else(|_| panic!("`{c}` should be a number"))
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("`{s}` should be a number triplet"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 22;
        let actual = solve(
            "\
7,7,2
7,1,3
12,0,4
15,5,3
24,1,6
28,5,5
40,3,3
40,8,2",
        );

        assert_eq!(expected, actual);
    }
}
