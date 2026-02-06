use anyhow::bail;
use std::{collections::VecDeque, ops::RangeInclusive};

fn parse_range(input: &str) -> anyhow::Result<RangeInclusive<i32>> {
    let Some((start, end)) = input.split_once('-') else {
        bail!("invalid range: {input}");
    };

    Ok(start.parse()?..=end.parse()?)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    enum Direction {
        Clockwise,
        Anticlockwise,
    }

    let ranges = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_range);

    let mut dequeue = VecDeque::from([1]);
    let mut direction = Direction::Clockwise;
    let mut start_index = 0;

    for range in ranges {
        let range = range?;
        match direction {
            Direction::Clockwise => {
                dequeue.extend(range);
                direction = Direction::Anticlockwise;
            }
            Direction::Anticlockwise => {
                range.for_each(|i| {
                    dequeue.push_front(i);
                    start_index += 1;
                });
                direction = Direction::Clockwise;
            }
        }
    }

    Ok(dequeue
        .iter()
        .cycle()
        .skip(start_index)
        .take(dequeue.len())
        .copied()
        .collect())
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const TURNS: usize = 20252025;
    let wheel = parse_input(input)?;

    let Some(&number) = wheel.get(TURNS % wheel.len()) else {
        bail!("empty wheel");
    };

    println!("Answer: {number}");

    Ok(())
}
