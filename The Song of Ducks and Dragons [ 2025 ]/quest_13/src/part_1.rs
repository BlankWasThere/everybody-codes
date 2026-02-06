use std::collections::VecDeque;

use anyhow::bail;

fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    enum Direction {
        Clockwise,
        Anticlockwise,
    }

    let numbers = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse());

    let mut dequeue = VecDeque::from([1]);
    let mut direction = Direction::Clockwise;
    let mut start_index = 0;

    for number in numbers {
        let number = number?;
        match direction {
            Direction::Clockwise => {
                dequeue.push_back(number);
                direction = Direction::Anticlockwise;
            }
            Direction::Anticlockwise => {
                dequeue.push_front(number);
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
    const TURNS: usize = 2025;
    let wheel = parse_input(input)?;

    let Some(&number) = wheel.get(TURNS % wheel.len()) else {
        bail!("empty wheel");
    };

    println!("Answer: {number}");

    Ok(())
}
