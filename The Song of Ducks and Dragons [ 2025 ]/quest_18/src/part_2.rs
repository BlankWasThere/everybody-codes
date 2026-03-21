use std::collections::HashMap;

use anyhow::{Context, Ok};

type Thickness = i64;
type PlantId = u32;
type Plants = HashMap<PlantId, (Thickness, Vec<Branch>)>;
type TestCase = u16;

#[derive(Debug)]
struct Branch {
    thickness: Thickness,
    connection: Option<PlantId>,
}

pub fn solve(input: &str) -> anyhow::Result<i64> {
    let (plants, last_plant, test_cases) = parse_input(input)?;

    test_cases
        .into_iter()
        .map(|case| energy_output(&plants, last_plant, case))
        .sum()
}

fn energy_output(plants: &Plants, last_plant: PlantId, test_case: TestCase) -> anyhow::Result<i64> {
    let &(thickness, ref branches) = plants
        .get(&last_plant)
        .context(format!("plant {last_plant} not found"))?;

    let mut incoming = 0;
    for branch in branches {
        let connection_energy = if let Some(plant_id) = branch.connection {
            energy_output(plants, plant_id, test_case)?
        } else {
            // 1 if included in test case, else 0
            (test_case & (0x1 << (last_plant - 1)) != 0) as _
        };

        incoming += branch.thickness * connection_energy;
    }

    Ok(if incoming < thickness { 0 } else { incoming })
}

fn parse_input(input: &str) -> anyhow::Result<(Plants, PlantId, Vec<TestCase>)> {
    // Normalize CRLF
    let input = input.replace("\r\n", "\n");

    let mut plants = HashMap::new();
    let mut last = None;

    let mut sections = input.trim().split("\n\n\n");

    let schema = sections.next().context("schema not found in input")?;
    for block in schema.split("\n\n") {
        let mut lines = block.lines().map(str::trim).filter(|s| !s.is_empty());
        let (plant_id, thickness) = {
            let Some(line) = lines.next() else {
                continue;
            };

            let mut words = line.split_whitespace();
            let parse_error = || format!("invalid line: `{line}`");

            let plant_id = words.nth(1).with_context(parse_error)?.parse()?;
            let thickness = words
                .last()
                .with_context(parse_error)?
                .strip_suffix(':')
                .with_context(parse_error)?
                .parse()?;

            (plant_id, thickness)
        };

        let mut branches = vec![];
        for line in lines {
            let is_free = line.contains("free");
            let branch = if is_free {
                let thickness = line
                    .split_whitespace()
                    .last()
                    .context(format!("invalid line: `{line}`"))?
                    .parse()?;

                Branch {
                    thickness,
                    connection: None,
                }
            } else {
                let mut words = line.split_whitespace();
                let parse_error = || format!("invalid line: `{line}`");

                let connection = words.nth(4).with_context(parse_error)?.parse()?;
                let thickness = words.last().with_context(parse_error)?.parse()?;

                Branch {
                    thickness,
                    connection: Some(connection),
                }
            };

            branches.push(branch);
        }

        last = Some(plant_id);
        plants.insert(plant_id, (thickness, branches));
    }

    let mut test_cases = vec![];

    let test_cases_str = sections.next().context("test cases not found in input")?;
    for line in test_cases_str.lines() {
        let numbers = line.split_whitespace().map(str::parse::<u16>);
        let mut test_case = 0;

        for (i, num) in numbers.enumerate() {
            test_case |= num? << i;
        }

        test_cases.push(test_case);
    }

    let last = last.context("empty input")?;

    Ok((plants, last, test_cases))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 324;
        let actual = solve(
            "\
Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}
