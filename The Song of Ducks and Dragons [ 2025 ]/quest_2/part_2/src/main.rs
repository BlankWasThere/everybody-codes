use simple_complex_numbers::Complex;

static USAGE_STR: &str = concat!("Usage: ", env!("CARGO_BIN_NAME"), " <A=value>");

struct Problem {
    value: Complex,
}

fn main() -> anyhow::Result<()> {
    let problem = parse_args()?;
    let solution = solve(problem);

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

    let name_value_pair = args.next().unwrap();
    let (name, value) = name_value_pair.split_once('=').ok_or(anyhow::anyhow!(
        "Invalid name value pair found `{}`.",
        name_value_pair
    ))?;

    if name != "A" {
        return Err(anyhow::anyhow!("The name must be 'A', not `{}`", name));
    }

    if !value.starts_with('[') || !value.ends_with("]") || !value.contains(',') {
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
            s.parse::<i64>() // Convert to i64
                .map_err(|_| anyhow::anyhow!("Invalid number `{}`.", s))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    if numbers.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid value `{}`; It must be in the format [int,int].",
            value
        ));
    }

    Ok(Problem {
        value: Complex::new(numbers[0], numbers[1]),
    })
}

fn solve(Problem { value }: Problem) -> usize {
    let start = value;
    let end = start + Complex::new(1000, 1000);
    let mut can_be_engraved: usize = 0;

    let points = 101;
    let step = 1000 / (points - 1);

    for y in (start.imag()..=end.imag()).into_iter().step_by(step) {
        'outer: for x in (start.real()..=end.real()).into_iter().step_by(step) {
            let mut check_result = Complex::new(0, 0);

            // Perform 100 cycles for each point
            for _ in 0..100 {
                check_result *= check_result;
                check_result /= Complex::new(100000, 100000);
                check_result += Complex::new(x, y);

                for n in [check_result.real(), check_result.imag()] {
                    if n < -1000000 || n > 1000000 {
                        continue 'outer;
                    }
                }
            }

            can_be_engraved += 1;
        }
    }

    can_be_engraved
}
