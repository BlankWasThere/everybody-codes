pub fn solve(input: &str) -> u64 {
    let groups = parse_input(input);
    let mut sum = 0;

    let max_len = groups[0]
        .iter()
        .copied()
        .map(|(a, b)| a.max(b))
        .max()
        .unwrap() as usize;

    for group in groups {
        let mut graph = Vec::with_capacity(max_len);
        let mut indegrees = vec![0; max_len];
        let mut connections = vec![vec![]; max_len];

        for &(greater, smaller) in &group {
            indegrees[(smaller - 1) as usize] += 1;
            connections[(greater - 1) as usize].push(smaller);
        }

        while graph.len() < max_len {
            let mut least_idx = -1;
            for i in 0..indegrees.len() {
                if indegrees[i] < 0 {
                    continue;
                }

                if least_idx == -1 || indegrees[i] < indegrees[least_idx as usize] {
                    least_idx = i as _;
                }
            }

            indegrees[least_idx as usize] -= 1;
            graph.push(least_idx + 1);

            for &connection in &connections[least_idx as usize] {
                indegrees[(connection - 1) as usize] -= 1;
            }
        }

        sum += graph
            .into_iter()
            .rev()
            .enumerate()
            .map(|(idx, v)| v as u64 * 10u64.pow(idx as _))
            .sum::<u64>();
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
3>1 1>2 1<5 4<2 3>5
2>1 2<5 3>1 4>3 2<4 5<3
5>1 3<4 1>2 3>5",
        );
        let expected = 122157;

        assert_eq!(output, expected);
    }

    #[test]
    fn test_2() {
        let output = solve(
            "\
3<2 1>2 5<4 4<3",
        );
        let expected = 12345;

        assert_eq!(output, expected);
    }
}
