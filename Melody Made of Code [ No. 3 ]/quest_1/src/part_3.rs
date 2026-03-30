use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let scales = parse_input(input);
    let mut categories = HashMap::new();

    for (id, rgb) in scales {
        let [red, green, blue, shine] = rgb;

        let red = component_to_integer(red);
        let green = component_to_integer(green);
        let blue = component_to_integer(blue);
        let shine = component_to_integer(shine);

        let dominant_color = if red > green && red > blue {
            0
        } else if green > red && green > blue {
            1
        } else if blue > red && blue > green {
            2
        } else {
            continue;
        };

        let is_shiny = if shine >= 33 {
            true
        } else if shine <= 30 {
            false
        } else {
            continue;
        };

        categories
            .entry((dominant_color, is_shiny))
            .or_insert(vec![])
            .push(id);
    }

    let (_, ids) = categories
        .into_values()
        .map(|v| (v.len(), v))
        .max()
        .unwrap();

    ids.into_iter().sum()
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
        let expected = 292320;
        let actual = solve(
            "\
15437:rRrrRR gGGGGG BBBBBB sSSSSS
94682:RrRrrR gGGggG bBBBBB ssSSSs
56513:RRRrrr ggGGgG bbbBbb ssSsSS
76346:rRRrrR GGgggg bbbBBB ssssSs
87569:rrRRrR gGGGGg BbbbbB SssSss
44191:rrrrrr gGgGGG bBBbbB sSssSS
49176:rRRrRr GggggG BbBbbb sSSssS
85071:RRrrrr GgGGgg BBbbbb SSsSss
44303:rRRrrR gGggGg bBbBBB SsSSSs
94978:rrRrRR ggGggG BBbBBb SSSSSS
26325:rrRRrr gGGGgg BBbBbb SssssS
43463:rrrrRR gGgGgg bBBbBB sSssSs
15059:RRrrrR GGgggG bbBBbb sSSsSS
85004:RRRrrR GgGgGG bbbBBB sSssss
56121:RRrRrr gGgGgg BbbbBB sSsSSs
80219:rRRrRR GGGggg BBbbbb SssSSs",
        );

        assert_eq!(expected, actual);
    }
}
