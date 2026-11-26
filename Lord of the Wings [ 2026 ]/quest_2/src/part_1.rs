pub fn solve(input: &str) -> u32 {
    let groups = parse_input(input);
    let mut sum = 0;

    for group in groups {
        let mut powers_of_ten = [0; 5];

        for (a, _) in group {
            powers_of_ten[a as usize - 1] += 1;
        }

        sum += powers_of_ten
            .into_iter()
            .enumerate()
            .map(|(idx, exp)| (idx + 1) as u32 * 10u32.pow(exp))
            .sum::<u32>()
    }

    sum
}

fn parse_input(input: &str) -> Vec<Vec<(u32, u32)>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| {
                    let [first, op, second] = s
                        .chars()
                        .collect::<Vec<_>>()
                        .try_into()
                        .expect("input should be two numbers with separated by < or >");

                    let (first, second) = (
                        first
                            .to_digit(10)
                            .unwrap_or_else(|| panic!("expected digit, found {first}")),
                        second
                            .to_digit(10)
                            .unwrap_or_else(|| panic!("expected digit, found {second}")),
                    );

                    if op == '>' {
                        (first, second)
                    } else if op == '<' {
                        (second, first)
                    } else {
                        panic!("invalid operator: `{op}`");
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let output = solve(
            "\
4<3 2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
4>1 2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 2<4 3>2 3<4 1>2 3>5 1<3 4>5",
        );
        let expected = 122157;

        assert_eq!(output, expected);
    }
}
