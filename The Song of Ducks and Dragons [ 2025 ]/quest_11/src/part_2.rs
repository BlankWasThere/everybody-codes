fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let mut columns = parse_input(input)?;

    let mut rounds = 0;
    let mut phase = 0;

    loop {
        rounds += 1;

        match phase {
            // First phase
            0 => {
                let mut moved = false;

                for (i, j) in (0..columns.len()).zip(1..columns.len()) {
                    if columns[i] > columns[j] {
                        columns[j] += 1;
                        columns[i] -= 1;

                        moved = true;
                    }
                }

                // No more movement = next phase
                if !moved {
                    phase = 1;
                    rounds -= 1;
                }
            }
            // Second phase
            1 => {
                let mut moved = false;

                for (i, j) in (0..columns.len()).zip(1..columns.len()) {
                    if columns[j] > columns[i] {
                        columns[i] += 1;
                        columns[j] -= 1;

                        moved = true;
                    }
                }

                // No more movement = completed
                if !moved {
                    rounds -= 1;
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("Answer: {rounds}");

    Ok(())
}
