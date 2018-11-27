day!(
    day16,
    "https://adventofcode.com/2015/day/16/input",
    part1,
    part2
);

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Sue {
    index: usize,
    properties: HashMap<String, usize>,
}

fn parse_line(line: &str) -> Result<Sue> {
    lazy_static! {
        static ref MAIN: Regex = Regex::new(r"^Sue (?P<index>\d+): (?P<props>.+)$").unwrap();
        static ref PROP: Regex = Regex::new(r"(?P<key>[[:alpha:]]+): (?P<value>\d+)(, )?").unwrap();
    }

    let captures = MAIN.captures(line).ok_or(Error::Input("invalid format"))?;
    let index = captures["index"].parse().unwrap();
    let props = captures.name("props").unwrap().as_str();
    let properties = PROP
        .captures_iter(props)
        .map(|capture| {
            Ok((
                capture.name("key").unwrap().as_str().to_owned(),
                capture["value"].parse()?,
            ))
        })
        .collect::<Result<HashMap<String, usize>>>()?;

    Ok(Sue { index, properties })
}

const KNOWN_PROPS: [(&'static str, usize); 10] = [
    ("children", 3),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1),
];

fn part1(input: String) -> Result<usize> {
    let mut sues: Vec<Sue> = input.lines().map(parse_line).collect::<Result<_>>()?;
    sues.retain(|sue| {
        for &(key, value) in KNOWN_PROPS.iter() {
            if let Some(&sue_value) = sue.properties.get(key) {
                if sue_value != value {
                    return false;
                }
            }
        }
        true
    });

    if sues.len() != 1 {
        return Err(Error::Input("no sues matching the properties found"));
    }

    Ok(sues[0].index)
}

fn part2(input: String) -> Result<usize> {
    let mut sues: Vec<Sue> = input.lines().map(parse_line).collect::<Result<_>>()?;
    sues.retain(|sue| {
        for &(key, value) in KNOWN_PROPS.iter() {
            if let Some(&sue_value) = sue.properties.get(key) {
                if !match key {
                    "cats" | "trees" => sue_value > value,
                    "pomeranians" | "goldfish" => sue_value < value,
                    _ => sue_value == value,
                } {
                    return false;
                }
            }
        }
        true
    });

    if sues.len() != 1 {
        return Err(Error::Input("no sues matching the properties found"));
    }

    Ok(sues[0].index)
}

#[test]
fn day16_test() {
    assert_eq!(
        parse_line("Sue 1: cars: 9, akitas: 3, goldfish: 0").unwrap(),
        Sue {
            index: 1,
            properties: [
                ("cars".to_owned(), 9),
                ("akitas".to_owned(), 3),
                ("goldfish".to_owned(), 0)
            ]
            .iter()
            .cloned()
            .collect()
        }
    );
}
