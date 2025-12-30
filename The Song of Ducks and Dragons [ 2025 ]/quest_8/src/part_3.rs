fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .trim()
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| Ok(s.parse()?))
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    const TOTAL: u32 = 256;

    let numbers = parse_input(input)?;
    let mut max_cuts = 0;

    for prev in 1..=TOTAL {
        for next in 1..=TOTAL {
            let mut count = 0;

            for (&chk_prev, &chk_next) in numbers.iter().zip(numbers.iter().skip(1)) {
                if ((prev < chk_prev && chk_prev < next) && (chk_next < prev || chk_next > next))
                    || ((prev < chk_next && chk_next < next)
                        && (chk_prev < prev || chk_prev > next))
                {
                    count += 1;
                }
            }

            max_cuts = max_cuts.max(count)
        }
    }

    println!("Answer: {max_cuts}");

    Ok(())
}
