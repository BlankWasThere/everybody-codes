use std::collections::HashMap;

use anyhow::{Context, Ok, ensure};

type Thickness = i32;
type PlantId = u32;
type Plants = HashMap<PlantId, (Thickness, Vec<Branch>)>;
type TestCase = Vec<u8>;

#[derive(Debug)]
struct Branch {
    thickness: Thickness,
    connection: Option<PlantId>,
}

pub fn solve(input: &str) -> anyhow::Result<i32> {
    let (plants, last_plant, test_cases) = parse_input(input)?;

    let mut best_case = vec![];

    for id in 1..=last_plant {
        let (_, branches) = plants.get(&id).unwrap();
        for branch in branches {
            if let Some(plant_id) = branch.connection {
                if ((plant_id - 1) as usize) < best_case.len() && branch.thickness > 0 {
                    best_case[(plant_id - 1) as usize] = 1;
                }
            } else {
                best_case.push(0);
            }
        }
    }

    ensure!(best_case.len() == test_cases[0].len(), "invalid test cases");

    let max = energy_output(&plants, last_plant, &best_case).unwrap();

    let mut energy_difference = 0;
    for case in test_cases {
        let output = energy_output(&plants, last_plant, &case)?;
        if output > 0 {
            energy_difference += max - output;
        }
    }

    Ok(energy_difference)
}

fn energy_output(
    plants: &Plants,
    last_plant: PlantId,
    test_case: &TestCase,
) -> anyhow::Result<i32> {
    let &(thickness, ref branches) = plants
        .get(&last_plant)
        .context(format!("plant {last_plant} not found"))?;

    let mut incoming = 0;
    for branch in branches {
        let connection_energy = if let Some(plant_id) = branch.connection {
            energy_output(plants, plant_id, test_case)?
        } else {
            // 1 if included in test case, else 0
            test_case[last_plant as usize - 1] as _
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
        // let numbers = line.split_whitespace().map(str::parse::<u16>);
        let test_case = line
            .split_whitespace()
            .map(|s| Ok(s.parse()?))
            .collect::<anyhow::Result<Vec<_>>>()?;

        test_cases.push(test_case);
    }

    let last = last.context("empty input")?;

    Ok((plants, last, test_cases))
}
