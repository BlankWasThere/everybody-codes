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

    let mut checksum = 0;
    let mut rounds = 0;
    let mut phase = 0;

    while rounds < 10 {
        rounds += 1;

        match phase {
            // First phase
            0 => {
                checksum = 0;
                let mut moved = false;

                for (i, j) in (0..columns.len()).zip(1..columns.len()) {
                    if columns[i] > columns[j] {
                        columns[j] += 1;
                        columns[i] -= 1;

                        moved = true;
                    }

                    checksum += (i + 1) as u32 * columns[i];
                }

                checksum += columns.len() as u32 * columns.last().unwrap_or(&0);

                // No more movement = next phase
                if !moved {
                    rounds -= 1;
                    phase = 1;
                }
            }
            // Second phase
            1 => {
                checksum = 0;
                let mut moved = false;

                for (i, j) in (0..columns.len()).zip(1..columns.len()) {
                    if columns[j] > columns[i] {
                        columns[i] += 1;
                        columns[j] -= 1;

                        moved = true;
                    }

                    checksum += (i + 1) as u32 * columns[i];
                }

                checksum += columns.len() as u32 * columns.last().unwrap_or(&0);

                // No more movement = completed
                if !moved {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("Answer: {checksum}");

    Ok(())
}
