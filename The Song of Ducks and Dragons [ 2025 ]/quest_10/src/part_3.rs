use anyhow::{Ok, bail, ensure};
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct BoardInfo {
    size_x: i32,
    size_y: i32,
    dragon: Coord,
    sheeps: Vec<Coord>,
    hideouts: HashSet<Coord>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Turn {
    Dragon,
    Sheeps,
}

impl Coord {
    pub fn contained_within(&self, size_x: i32, size_y: i32) -> bool {
        self.x >= 0 && self.x < size_x && self.y >= 0 && self.y < size_y
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_inputs(input: &str) -> anyhow::Result<BoardInfo> {
    let clean_lines = input.lines().map(str::trim).filter(|s| !s.is_empty());
    let mut dragon = None;
    let mut sheeps = Vec::new();
    let mut hideouts = HashSet::new();
    let mut size_x = None;
    let mut size_y = 0;

    for (y, line) in clean_lines.enumerate() {
        size_y += 1;
        let mut curr_size_x = 0;

        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;

            curr_size_x += 1;

            match c {
                'S' => {
                    sheeps.push(Coord { x, y });
                }
                'D' => {
                    if dragon.is_none() {
                        dragon = Some(Coord { x, y })
                    } else {
                        bail!("multiple dragons found")
                    }
                }
                '#' => {
                    hideouts.insert(Coord { x, y });
                }
                '.' => (),
                other => bail!("invalid character: `{other}`"),
            }
        }

        if let Some(n) = size_x {
            ensure!(n == curr_size_x);
        } else {
            size_x = Some(curr_size_x);
        }
    }

    let Some(dragon) = dragon else {
        bail!("dragon not found");
    };

    let Some(size_x) = size_x else {
        bail!("empty board");
    };

    sheeps.sort_unstable();

    Ok(BoardInfo {
        size_x,
        size_y,
        dragon,
        sheeps,
        hideouts,
    })
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let mut board = parse_inputs(input)?;
    let wins = count_wins(&mut board, &mut HashMap::new(), Turn::Sheeps);

    println!("Answer: {wins}");

    Ok(())
}

fn count_wins(
    board: &mut BoardInfo,
    memo: &mut HashMap<(Coord, Vec<Coord>, Turn), u64>,
    turn: Turn,
) -> u64 {
    let mut wins = 0;

    if let Some(&v) = memo.get(&(board.dragon, board.sheeps.clone(), turn)) {
        return v;
    }

    match turn {
        Turn::Dragon => {
            for (x, y) in [
                (-1, -2),
                (1, -2),
                (2, -1),
                (2, 1),
                (1, 2),
                (-1, 2),
                (-2, 1),
                (-2, -1),
            ] {
                let delta = Coord { x, y };
                let next_pos = board.dragon + delta;

                if next_pos.contained_within(board.size_x, board.size_y) {
                    let curr_pos = board.dragon;
                    board.dragon = next_pos;

                    {
                        // Dragon eats sheep
                        let mut removed_sheep = None;
                        if !board.hideouts.contains(&board.dragon)
                            && let Some(i) = board.sheeps.iter().position(|s| s == &board.dragon)
                        {
                            removed_sheep = Some(board.sheeps.remove(i));
                        }

                        wins += count_wins(board, memo, Turn::Sheeps);

                        // Backtrack
                        if let Some(sheep) = removed_sheep {
                            board.sheeps.push(sheep);
                            board.sheeps.sort_unstable();
                        }
                    }

                    board.dragon = curr_pos;
                }
            }
        }
        Turn::Sheeps => {
            if board.sheeps.is_empty() {
                return 1;
            }

            let mut has_legal_move = false;

            for i in 0..board.sheeps.len() {
                board.sheeps[i].y += 1;

                if board.hideouts.contains(&board.sheeps[i]) || board.sheeps[i] != board.dragon {
                    has_legal_move = true;

                    if board.sheeps[i].contained_within(board.size_x, board.size_y) {
                        wins += count_wins(board, memo, Turn::Dragon);
                    }
                }

                board.sheeps[i].y -= 1;
            }

            if !has_legal_move {
                wins += count_wins(board, memo, Turn::Dragon);
            }
        }
    }

    memo.insert((board.dragon, board.sheeps.clone(), turn), wins);
    wins
}
