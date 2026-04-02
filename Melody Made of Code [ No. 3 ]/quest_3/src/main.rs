use std::{
    io::{self, ErrorKind},
    time::{Duration, Instant},
};

mod part_1;
mod part_2;
mod part_3;

fn read_input(path: &str) -> io::Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

fn main() {
    // Part 1
    if let Some(input) = read_input("input_1.txt").expect("should be able to read input 1") {
        println!("========== Part 1 ==========");
        let (answer, duration) = timeit(|| part_1::solve(&input));
        println!("Answer ({duration:?}): {answer}");
    }

    // Part 2
    if let Some(input) = read_input("input_2.txt").expect("should be able to read input 2") {
        println!("========== Part 2 ==========");
        let (answer, duration) = timeit(|| part_2::solve(&input));
        println!("Answer ({duration:?}): {answer}");
    }

    // Part 3
    if let Some(input) = read_input("input_3.txt").expect("should be able to read input 3") {
        println!("========== Part 3 ==========");
        let (answer, duration) = timeit(|| part_3::solve(&input));
        println!("Answer ({duration:?}): {answer}");
    }
}

fn timeit<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
    T: std::fmt::Display,
{
    let start = Instant::now();
    (f(), start.elapsed())
}
