use std::cmp::Reverse;

pub fn solve(input: &str) -> u32 {
    let scales = parse_input(input);

    let (.., id) = scales
        .into_iter()
        .map(|(id, rgb)| {
            let [red, green, blue, shine] = rgb;

            let red = component_to_integer(red);
            let green = component_to_integer(green);
            let blue = component_to_integer(blue);
            let shine = component_to_integer(shine);

            let color_sum = red + green + blue;

            (shine, Reverse(color_sum), id)
        })
        .max()
        .unwrap();

    id
}

fn component_to_integer(component: &str) -> u8 {
    let mut result = 0;

    for c in component.chars() {
        result <<= 1;
        result |= c.is_uppercase() as u8;
    }

    result
}

fn parse_input(input: &str) -> Vec<(u32, [&str; 4])> {
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
        let expected = 2456;
        let actual = solve(
            "\
2456:rrrrrr ggGgGG bbbbBB sSsSsS
7689:rrRrrr ggGggg bbbBBB ssSSss
3145:rrRrRr gggGgg bbbbBB sSsSsS
6710:rrrRRr ggGGGg bbBBbB ssSSss",
        );

        assert_eq!(expected, actual);
    }
}
