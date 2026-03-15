fn parse_input(input: &str) -> anyhow::Result<Vec<u64>> {
    input.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

pub fn solve(input: &str) -> anyhow::Result<u64> {
    const TOTAL_BLOCKS: u128 = 202520252025000;
    let wall_segment = parse_input(input)?;
    let spell = extract_spell(wall_segment);

    let mut upper = u64::MAX;
    let mut lower = u64::MIN;

    let mut columns = 0;
    while lower <= upper {
        let mid = lower + (upper - lower) / 2;
        let mid_blocks = blocks_for_columns(mid, &spell);

        match mid_blocks.cmp(&TOTAL_BLOCKS) {
            std::cmp::Ordering::Equal => {
                columns = mid;
                break;
            }
            std::cmp::Ordering::Less => {
                columns = mid;
                lower = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                upper = mid - 1;
            }
        }
    }

    Ok(columns)
}

fn extract_spell(mut wall: Vec<u64>) -> Vec<u64> {
    let mut result = vec![];

    while let Some(step) = wall.iter().position(|&n| n > 0).map(|n| n + 1) {
        wall.iter_mut()
            .skip(step - 1)
            .step_by(step)
            .for_each(|n| *n -= 1);

        result.push(step as u64);
    }

    assert!(wall.iter().all(|&n| n == 0));
    result
}

fn blocks_for_columns(columns: u64, spell: &[u64]) -> u128 {
    spell.iter().copied().map(|n| (columns / n) as u128).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = solve("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2").unwrap();
        let expected = 94439495762954;

        assert_eq!(expected, actual);
    }
}
