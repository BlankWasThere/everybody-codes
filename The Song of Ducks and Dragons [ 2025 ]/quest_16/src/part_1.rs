fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

pub fn solve(input: &str) -> anyhow::Result<u32> {
    const LENGTH: u32 = 90;
    let pattern = parse_input(input)?;

    Ok(pattern.into_iter().map(|n| LENGTH / n).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = solve("1,2,3,5,9").unwrap();
        let expected = 193;

        assert_eq!(expected, actual);
    }
}
