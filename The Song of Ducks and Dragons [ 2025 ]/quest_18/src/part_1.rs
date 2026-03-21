use std::collections::HashMap;

use anyhow::{Context, Ok};

type Thickness = u32;
type PlantId = u32;
type Plants = HashMap<PlantId, (Thickness, Vec<Branch>)>;

#[derive(Debug)]
struct Branch {
    thickness: Thickness,
    connection: Option<PlantId>,
}

pub fn solve(input: &str) -> anyhow::Result<u32> {
    let (plants, last_plant) = parse_input(input)?;

    energy_output(&plants, last_plant)
}

fn energy_output(plants: &Plants, last_plant: PlantId) -> anyhow::Result<u32> {
    let &(thickness, ref branches) = plants
        .get(&last_plant)
        .context(format!("plant {last_plant} not found"))?;

    let mut incoming = 0;
    for branch in branches {
        let connection_energy = if let Some(plant_id) = branch.connection {
            energy_output(plants, plant_id)?
        } else {
            1
        };

        incoming += branch.thickness * connection_energy
    }

    Ok(if incoming < thickness { 0 } else { incoming })
}

fn parse_input(input: &str) -> anyhow::Result<(Plants, PlantId)> {
    // Normalize CRLF
    let input = input.replace("\r\n", "\n");

    let mut plants = HashMap::new();
    let mut last = None;

    for block in input.trim().split("\n\n") {
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

    let last = last.context("empty input")?;

    Ok((plants, last))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 774;
        let actual = solve(
            "\
Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}
