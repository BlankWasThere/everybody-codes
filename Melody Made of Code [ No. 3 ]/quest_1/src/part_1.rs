pub fn solve(input: &str) -> u32 {
    let scales = parse_input(input);
    let mut sum = 0;

    for (id, rgb) in scales {
        let [red, green, blue] = rgb;

        let red = component_to_integer(red);
        let green = component_to_integer(green);
        let blue = component_to_integer(blue);

        if green > red && green > blue {
            sum += id;
        }
    }

    sum
}

fn component_to_integer(component: &str) -> u8 {
    let mut result = 0;

    for c in component.chars() {
        result <<= 1;
        result |= c.is_uppercase() as u8;
    }

    result
}

fn parse_input(input: &str) -> Vec<(u32, [&str; 3])> {
    input
        .lines()
        .map(str::trim)
        .map(|e| {
            let (id, colors) = e.split_once(':').unwrap();
            let rgb = colors
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .expect("should have been three values");

            (id.parse().expect("should be a valid number"), rgb)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 9166;
        let actual = solve(
            "\
2456:rrrrrr ggGgGG bbbbBB
7689:rrRrrr ggGggg bbbBBB
3145:rrRrRr gggGgg bbbbBB
6710:rrrRRr ggGGGg bbBBbB",
        );

        assert_eq!(expected, actual);
    }
}
