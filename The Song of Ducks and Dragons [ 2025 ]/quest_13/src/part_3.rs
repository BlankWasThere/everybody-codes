use anyhow::bail;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct SimpleRange {
    start: u64,
    end: u64,
}

fn parse_range(input: &str) -> anyhow::Result<SimpleRange> {
    let Some((start, end)) = input.split_once('-') else {
        bail!("invalid range: {input}");
    };

    Ok(SimpleRange {
        start: start.parse()?,
        end: end.parse()?,
    })
}

fn parse_input(input: &str) -> anyhow::Result<Vec<SimpleRange>> {
    enum Direction {
        Clockwise,
        Anticlockwise,
    }

    let ranges = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_range);

    let mut dequeue = VecDeque::from([SimpleRange { start: 1, end: 1 }]);
    let mut direction = Direction::Clockwise;
    let mut start_index = 0;

    for range in ranges {
        let range = range?;
        match direction {
            Direction::Clockwise => {
                dequeue.push_back(range);
                direction = Direction::Anticlockwise;
            }
            Direction::Anticlockwise => {
                dequeue.push_front(SimpleRange {
                    start: range.end,
                    end: range.start,
                });
                start_index += 1;
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
    let mut turns = 202520252025 + 1;
    let wheel = parse_input(input)?;

    let total = wheel
        .iter()
        .fold(0, |acc, e| acc + e.end.abs_diff(e.start) + 1);
    turns %= total;

    let mut number = None;

    for range in wheel {
        let size = range.end.abs_diff(range.start) + 1;
        if turns > size {
            turns -= size;
        } else {
            if range.end >= range.start {
                number = Some(range.start + (turns - 1));
            } else {
                number = Some(range.start - (turns - 1));
            };
            break;
        }
    }

    let Some(number) = number else {
        bail!("empty wheel");
    };

    println!("Answer: {number}");

    Ok(())
}
