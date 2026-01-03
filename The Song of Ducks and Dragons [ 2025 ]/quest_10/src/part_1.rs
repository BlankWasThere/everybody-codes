use anyhow::bail;
use std::collections::HashSet;

fn parse_input(input: &str) -> anyhow::Result<((usize, usize), Vec<Vec<bool>>)> {
    let mut dragon = None;

    let board = input
        .trim()
        .lines()
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Ok(false),
                    'S' => Ok(true),
                    'D' => {
                        if dragon.is_none() {
                            dragon = Some((row, col))
                        } else {
                            bail!("Multiple dragons found.")
                        };
                        Ok(false)
                    }
                    other => bail!("Invalid character found `{other}`"),
                })
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let Some(dragon) = dragon else {
        bail!("Dragon not found.")
    };

    Ok((dragon, board))
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const MOVES: u32 = 4;
    let (dragon, board) = parse_input(input)?;

    let mut visited = HashSet::new();
    let mut sheeps = HashSet::new();

    let mut stack = vec![((dragon.0 as isize, dragon.1 as isize), MOVES)];
    while let Some(((row, col), remaining_moves)) = stack.pop() {
        if row < 0
            || col < 0
            || row as usize >= board.len()
            || col as usize >= board[row as usize].len()
        {
            continue;
        }

        if !visited.insert(((row, col), remaining_moves)) {
            continue;
        }

        if board[row as usize][col as usize] {
            sheeps.insert((row, col));
        }

        if remaining_moves == 0 {
            continue;
        }

        stack.extend([
            ((row - 2, col - 1), remaining_moves - 1),
            ((row - 2, col + 1), remaining_moves - 1),
            ((row + 2, col - 1), remaining_moves - 1),
            ((row + 2, col + 1), remaining_moves - 1),
            ((row - 1, col - 2), remaining_moves - 1),
            ((row + 1, col - 2), remaining_moves - 1),
            ((row - 1, col + 2), remaining_moves - 1),
            ((row + 1, col + 2), remaining_moves - 1),
        ]);
    }

    println!("Answer: {}", sheeps.len());

    Ok(())
}
