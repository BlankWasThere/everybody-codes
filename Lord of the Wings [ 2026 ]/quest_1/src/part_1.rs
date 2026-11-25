pub fn solve(input: &str) -> u32 {
    let cones_vec = parse_input(input.trim());

    let mut required = 0;
    for cones in cones_vec {
        let side = (cones as f32).sqrt().ceil() as u32;
        let total = side * side;

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
        let expected = 11;

        assert_eq!(output, expected)
    }
}
