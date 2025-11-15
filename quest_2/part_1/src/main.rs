use simple_complex_numbers::Complex;
use std::collections::HashMap;

static USAGE_STR: &str = concat!("Usage: ", env!("CARGO_BIN_NAME"), " <name=value...;>");

struct Problem {
    map: HashMap<String, Complex>,
}

fn main() -> anyhow::Result<()> {
    let problem = parse_args()?;
    let solution = solve(problem)?;

    println!("The result is {}", solution);

    Ok(())
}

fn parse_args() -> anyhow::Result<Problem> {
    let mut args = std::env::args().skip(1); // Skipping first because it will be the program name in most systems.
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            "Invalid number of arguments.\n\n{}",
            USAGE_STR
        ));
    }

    let name_value_pairs = args
        .next()
        .unwrap()
        .split(';')
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(trimmed.to_owned())
            } else {
                None
            }
        })
        .map(|s| {
            let (name, value) = s
                .split_once('=')
                .ok_or(anyhow::anyhow!("Invalid name value pair found `{}`.", s))?;

            if !value.starts_with('[') || !value.ends_with("]") || value.len() < 3 {
                return Err(anyhow::anyhow!(
                    "Invalid value `{}`; It must be in the format [int,int].",
                    value
                ));
            }

            let value = &value[1..value.len() - 1]; // Remove '[' and ']'
            let numbers = value
                .split(',')
                .filter_map(|s| {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        Some(trimmed)
                    } else {
                        None
                    }
                })
                .map(|s| {
                    Ok(s.parse::<i64>() // Convert to i64
                        .map_err(|_| anyhow::anyhow!("Invalid number `{}`.", s))?)
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            if numbers.len() != 2 {
                return Err(anyhow::anyhow!("Invalid name value pair `{}`.", s));
            }

            Ok((name.to_owned(), Complex::new(numbers[0], numbers[1])))
        })
        .collect::<anyhow::Result<HashMap<_, _>>>()?;

    Ok(Problem {
        map: name_value_pairs,
    })
}

fn solve(problem: Problem) -> anyhow::Result<Complex> {
    let mut result = Complex::new(0, 0);

    for _ in 0..3 {
        result *= result;
        result /= Complex::new(10, 10);
        result += *problem
            .map
            .get("A")
            .ok_or(anyhow::anyhow!("Value 'A' not found."))?;
    }

    Ok(result)
}
