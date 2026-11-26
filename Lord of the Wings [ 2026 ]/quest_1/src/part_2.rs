pub fn solve(input: &str) -> u32 {
    let cones_vec = parse_input(input.trim());

    cones_vec
        .into_iter()
        .map(|cones| {
            if cones <= 8 {
                8 - cones
            } else {
                let next_dim = ((cones + 4) as f32 / 4.0).ceil() as u32;
                next_dim.pow(2) - (next_dim - 2).pow(2) - cones
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|e| e.parse().expect("value should be an integer"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let output = solve("3,11,12,16,24,25");
        let expected = 9;

        assert_eq!(output, expected)
    }
}
