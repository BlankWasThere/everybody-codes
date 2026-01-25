fn parse_input(input: &str) -> anyhow::Result<Vec<u64>> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let columns = parse_input(input)?;

    // This code assumes that phase 1 is skipped
    let mean = columns.iter().sum::<u64>() / columns.len() as u64;
    let rounds = columns
        .into_iter()
        .filter(|&n| n < mean)
        .map(|n| mean - n)
        .sum::<u64>();

    println!("Answer: {rounds}");

    Ok(())
}
