use std::{
    io::ErrorKind,
    time::{Duration, Instant},
};

mod part_1;
mod part_2;
mod part_3;

fn read_input(path: &str) -> anyhow::Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

fn main() -> anyhow::Result<()> {
    // Part 1
    if let Some(input) = read_input("input_1.txt")? {
        println!("========== Part 1 ==========");
        let (answer, duration) = timeit(|| part_1::solve(&input))?;
        println!("Answer ({:?}): {}", duration, answer);
    }

    // Part 2
    if let Some(input) = read_input("input_2.txt")? {
        println!("========== Part 2 ==========");
        let (answer, duration) = timeit(|| part_2::solve(&input))?;
        println!("Answer ({:?}): {}", duration, answer);
    }

    // Part 3
    if let Some(input) = read_input("input_3.txt")? {
        println!("========== Part 3 ==========");
        let (answer, duration) = timeit(|| part_3::solve(&input))?;
        println!("Answer ({:?}): {}", duration, answer);
    }

    Ok(())
}

fn timeit<F, T, E>(f: F) -> Result<(T, Duration), E>
where
    F: FnOnce() -> Result<T, E>,
    T: std::fmt::Display,
{
    let start = Instant::now();
    Ok((f()?, start.elapsed()))
}
