//! **Note:** This module assumes the floor is a 34 x 34 matrix
//! and the repeating pattern is an 8 x 8 matrix.

use std::collections::HashMap;

use anyhow::{bail, ensure};

const FLOOR_ORDER: usize = 34;
const PATTERN_ORDER: usize = 8;
type RepeatingPattern = [u8; PATTERN_ORDER];

fn parse_input(input: &str) -> anyhow::Result<RepeatingPattern> {
    let lines = input.lines().map(str::trim).filter(|s| !s.is_empty());
    let mut result = [0; PATTERN_ORDER];

    for (row, line) in lines.enumerate() {
        let mut current_tile = 0;

        for (i, c) in line.chars().enumerate() {
            ensure!(
                i < PATTERN_ORDER,
                "Your input line exceeds 8 characters. This code assumes {PATTERN_ORDER} x {PATTERN_ORDER} grid."
            );

            match c {
                '#' => current_tile |= 0x1 << (PATTERN_ORDER - 1 - i),
                '.' => {}
                _ => bail!("invalid character: `{c}`"),
            }
        }

        result[row] = current_tile;
    }

    Ok(result)
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const TURNS: u32 = 1000000000;

    let pattern = parse_input(input)?;
    let mut floor = [0u64; FLOOR_ORDER];
    let mut count = 0u64;

    let mut fast_forwarded = false;
    let mut visited = HashMap::new();
    let mut curr_round = 0;
    let mut last_round_pattern_matched = false;

    while curr_round < TURNS {
        if !fast_forwarded && let Some(&(round, round_count)) = visited.get(&floor) {
            fast_forwarded = true;

            let round_diff = curr_round - round;
            assert_ne!(round_diff, 0, "how could this even happen?");

            let jumps = (TURNS - round) / round_diff;

            curr_round = round + round_diff * jumps;

            let count_diff = count - round_count;
            count = round_count + count_diff * jumps as u64;
        } else if last_round_pattern_matched {
            visited.insert(floor, (curr_round, count));
        }

        let mut curr_count = 0;

        let mut floor_update = floor;

        for row in 0..FLOOR_ORDER {
            for col in 0..FLOOR_ORDER {
                let mut should_be_active = floor[row] & (0x1 << col) == 0;

                for neighbour_status in get_diagonal_neighbours(floor, (row, col)) {
                    should_be_active ^= neighbour_status;
                }

                if should_be_active {
                    floor_update[row] |= 0x1 << col;
                    curr_count += 1;
                } else {
                    floor_update[row] &= !(0x1 << col);
                }
            }
        }

        floor = floor_update;

        last_round_pattern_matched = match_pattern(floor, pattern);
        if last_round_pattern_matched {
            count += curr_count;
        }

        curr_round += 1;
    }

    println!("Answer: {count}");
    Ok(())
}

fn match_pattern(floor: [u64; FLOOR_ORDER], pattern: [u8; PATTERN_ORDER]) -> bool {
    let start = (FLOOR_ORDER - PATTERN_ORDER) / 2;
    let end = start + PATTERN_ORDER;

    for row in start..end {
        for col in start..end {
            if (pattern[row - start] & (0x1 << (col - start)) != 0)
                != (floor[row] & (0x1 << col) != 0)
            {
                return false;
            }
        }
    }

    true
}

fn get_diagonal_neighbours(
    floor: [u64; FLOOR_ORDER],
    (row, col): (usize, usize),
) -> impl Iterator<Item = bool> {
    [(-1, -1), (-1, 1), (1, -1), (1, 1)]
        .into_iter()
        .filter_map(move |(dr, dc)| {
            if let (Some(tr), Some(tc)) = (row.checked_add_signed(dr), col.checked_add_signed(dc))
                && col < 34
                && let Some(&r) = floor.get(tr)
            {
                Some(r & (0x1 << tc) != 0)
            } else {
                None
            }
        })
}
