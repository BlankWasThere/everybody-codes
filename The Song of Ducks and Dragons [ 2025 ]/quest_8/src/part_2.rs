macro_rules! simple_min_max {
    ($x:expr, $y:expr) => {{
        let x = $x;
        let y = $y;
        if x > y { (y, x) } else { (x, y) }
    }};
}

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
    let numbers = parse_input(input)?;
    let mut count = 0;

    for (idx, (&prev, &next)) in numbers.iter().zip(numbers.iter().skip(1)).enumerate() {
        let (prev, next) = simple_min_max!(prev, next);

        for (&chk_prev, &chk_next) in numbers.iter().zip(numbers.iter().skip(1)).take(idx) {
            if ((prev < chk_prev && chk_prev < next) && (chk_next < prev || chk_next > next))
                || ((prev < chk_next && chk_next < next) && (chk_prev < prev || chk_prev > next))
            {
                count += 1;
            }
        }
    }

    println!("Answer: {count}");

    Ok(())
}
