fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

pub fn solve(input: &str) -> anyhow::Result<u64> {
    let mut pattern = parse_input(input)?;
    let mut spell = vec![];

    while let Some(step) = pattern.iter().position(|&n| n > 0).map(|n| n + 1) {
        pattern
            .iter_mut()
            .skip(step - 1)
            .step_by(step)
            .for_each(|n| *n -= 1);

        spell.push(step as u64);
    }

    assert!(pattern.iter().all(|&n| n == 0));

    Ok(spell.into_iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = solve("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2").unwrap();
        let expected = 270;

        assert_eq!(expected, actual);
    }
}
