use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
};
pub fn solve(input: &str) -> u32 {
    let input = parse_input(input);

    let mut xs = vec![0];
    let mut openings: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for &[x, y, height] in &input {
        if let Some(&last) = xs.last()
            && last != x
        {
            xs.push(x);
        }

        let opening = (y, y + height - 1);
        openings
            .entry(x)
            .and_modify(|v| v.push(opening))
            .or_insert(vec![opening]);
    }

    let mut pq = BTreeSet::from([(Reverse(0), 0, 0)]);
    while let Some((Reverse(x_idx), steps, curr_y)) = pq.pop_first() {
        if x_idx == xs.len() - 1 {
            return steps;
        }

        let curr_x = xs[x_idx];
        let next_x = xs[x_idx + 1];

        let allowed_y_delta = next_x - curr_x;
        let next_y_range = openings
            .get(&next_x)
            .unwrap()
            .iter()
            .flat_map(|&(y_start, y_end)| y_start..=y_end)
            .filter(|&next_y| next_y.abs_diff(curr_y) <= allowed_y_delta);

        for next_y in next_y_range {
            if !(next_x + next_y).is_multiple_of(2) {
                continue;
            }

            let y_delta = next_y as i32 - curr_y as i32;

            let (level_steps, mid_x) = if y_delta < 0 {
                (0u32, curr_x + y_delta.unsigned_abs())
            } else {
                (y_delta as _, curr_x + y_delta as u32)
            };

            let stability_steps = (next_x - mid_x) / 2;

            let next_steps = steps + level_steps + stability_steps;
            pq.insert((Reverse(x_idx + 1), next_steps, next_y));
        }
    }

    unreachable!();
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
