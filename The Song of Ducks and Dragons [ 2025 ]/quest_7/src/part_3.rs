use std::collections::{HashMap, HashSet};

type Rules = HashMap<char, Vec<char>>;

fn parse_input(input: &str) -> anyhow::Result<(Vec<String>, Rules)> {
    let mut lines = input.trim().lines();
    let prefixes = lines
        .next()
        .ok_or(anyhow::anyhow!("Missing prefixes in input."))?
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() {
                Some(s.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let rules = lines
        .filter_map(|s| {
            let s = s.trim();
            if !s.is_empty() { Some(s) } else { None }
        })
        .map(|s| {
            let (before, after) = s
                .split_once('>')
                .ok_or(anyhow::anyhow!("Invalid rule `{s}`"))?;

            let before = before
                .chars()
                .next()
                .ok_or(anyhow::anyhow!("Invalid rule `{s}`"))?;

            let after = after
                .split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    if !s.is_empty() { Some(s) } else { None }
                })
                .map(|s| {
                    s.chars()
                        .next()
                        .ok_or(anyhow::anyhow!("Invalid rule `{s}`"))
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            Ok((before, after))
        })
        .collect::<anyhow::Result<HashMap<_, _>>>()?;

    Ok((prefixes, rules))
}

fn check_prefix(prefix: &str, rules: &Rules) -> bool {
    for (prev, next) in prefix.chars().zip(prefix.chars().skip(1)) {
        if let Some(possible_nexts) = rules.get(&prev)
            && possible_nexts.contains(&next)
        {
            continue;
        }

        return false;
    }

    true
}

pub fn solve(input: &str) -> anyhow::Result<()> {
    let (prefixes, rules) = parse_input(input)?;
    let mut valid_names = HashSet::new();

    let mut valid_prefixes = prefixes
        .into_iter()
        .filter(|p| check_prefix(p, &rules))
        .collect::<Vec<_>>();

    while let Some(name) = valid_prefixes.pop() {
        if name.len() > 11 {
            continue;
        }

        if name.len() >= 7 && !valid_names.insert(name.clone()) {
            continue;
        }

        let last_letter = name.chars().last().expect("Unexpected empty name found.");
        if let Some(next_possible_letters) = rules.get(&last_letter) {
            for &next_letter in next_possible_letters {
                let mut new_name = name.clone();
                new_name.push(next_letter);

                valid_prefixes.push(new_name);
            }
        }
    }

    println!("Answer: {}", valid_names.len());

    Ok(())
}
