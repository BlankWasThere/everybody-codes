pub fn solve(input: &str) -> u32 {
    let cones_vec = parse_input(input.trim());

    let mut required = 0;
    for cones in cones_vec {
        let total = if cones <= 8 {
            8
        } else {
            let mut side = (cones as f32 + 1.0).sqrt().ceil() as u32;
            side += side.is_multiple_of(2) as u32;
            side * side - 1
        };

        required += total - cones;
    }

    required
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
        let expected = 61;

        assert_eq!(output, expected)
    }
}
