use anyhow::{bail, ensure};

type Floor = Vec<Vec<bool>>;

fn parse_input(input: &str) -> anyhow::Result<Floor> {
    let lines = input.lines().map(str::trim).filter(|s| !s.is_empty());
    let mut result: Floor = vec![];

    for line in lines {
        let mut tiles_in_current_line = vec![];

        for c in line.chars() {
            match c {
                '#' => {
                    tiles_in_current_line.push(true);
                }
                '.' => {
                    tiles_in_current_line.push(false);
                }
                _ => bail!("invalid character: `{c}`"),
            }
        }

        if let Some(v) = result.first() {
            // Ensure the floor is a rectangular matrix
            ensure!(v.len() == tiles_in_current_line.len())
        }

        result.push(tiles_in_current_line);
    }

    Ok(result)
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const TURNS: u32 = 10;

    let mut floor = parse_input(input)?;
    let mut count = 0;

    for _ in 0..TURNS {
        let mut floor_update = floor.clone();

        for row_idx in 0..floor.len() {
            for col_idx in 0..floor[row_idx].len() {
                let mut should_be_active = !floor[row_idx][col_idx];

                for (dr, dc) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                    if let (Some(tr), Some(tc)) = (
                        row_idx.checked_add_signed(dr),
                        col_idx.checked_add_signed(dc),
                    ) && let Some(r) = floor.get(tr)
                        && let Some(&v) = r.get(tc)
                        && v
                    {
                        // Invert the boolean
                        should_be_active ^= true;
                    }
                }

                floor_update[row_idx][col_idx] = should_be_active;
                count += should_be_active as u32;
            }
        }

        floor = floor_update;
    }

    println!("Answer: {count}");
    Ok(())
}
