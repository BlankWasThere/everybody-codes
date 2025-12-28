pub fn solve(input: &str) -> anyhow::Result<()> {
    const MAX_DISTANCE: usize = 1000;
    const REPETITIONS: usize = 1000;

    let chars = input.trim().chars().collect::<Vec<_>>();

    let one = calculate(&chars, 1, MAX_DISTANCE);
    let two = calculate(&chars, 2, MAX_DISTANCE);
    let diff = two - one;

    let count = one + diff * (REPETITIONS - 1) as u64;

    println!("Answer: {count}");
    Ok(())
}

fn calculate(chars: &[char], repetitions: usize, max_distance: usize) -> u64 {
    let mut count = 0_u64;

    for (i, &c) in chars
        .iter()
        .cycle()
        .take(chars.len() * repetitions)
        .enumerate()
    {
        if c.is_ascii_lowercase() {
            let li = i.saturating_sub(max_distance);
            let ri = (i + max_distance).min(chars.len() * repetitions - 1);

            count += chars
                .iter()
                .cycle()
                .skip(li)
                .take(ri - li + 1)
                .filter(|&&m| m.is_ascii_uppercase() && m.to_ascii_lowercase() == c)
                .count() as u64;
        }
    }

    count
}
