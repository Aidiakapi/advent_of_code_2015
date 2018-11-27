day!(
    day13,
    "https://adventofcode.com/2015/day/13/input",
    part1,
    part2
);

use permutohedron::Heap;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type People<'a> = HashSet<&'a str>;
type Happiness<'a> = HashMap<(&'a str, &'a str), isize>;

fn parse_happiness<'a>(input: &'a str) -> Result<(People<'a>, Happiness<'a>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^(?P<f>[[:alpha:]]+) would (?P<n>gain|lose) (?P<a>\d+) happiness units by sitting next to (?P<t>[[:alpha:]]+)\.$").unwrap();
    }
    let happiness: Happiness<'a> = RE
        .captures_iter(input)
        .map(|m| {
            let from = m.name("f").unwrap().as_str();
            let is_negative = &m["n"] == "lose";
            let amount: isize = m["a"].parse().unwrap();
            let to = m.name("t").unwrap().as_str();
            ((from, to), if is_negative { -amount } else { amount })
        })
        .collect();

    if happiness.is_empty() {
        return Err(Error::Input("expected any input"));
    }

    let mut people = HashSet::new();
    for (f, t) in happiness.keys() {
        people.insert(*f);
        if !happiness.contains_key(&(t, f)) {
            return Err(Error::Input("happiness should be specified both ways"));
        }
    }

    Ok((people, happiness))
}

fn compute_total_happiness<'a>((people, happiness): (People<'a>, Happiness<'a>)) -> isize {
    let mut people = people.into_iter().collect::<Vec<_>>();
    let heap = Heap::new(&mut people);
    heap
        .map(|permutation| {
            permutation
                .iter()
                .zip(
                    permutation
                        .iter()
                        .skip(1)
                        .chain(std::iter::once(&permutation[0])),
                )
                .filter_map(|(a, b)| {
                    if a == b {
                        None
                    } else {
                        Some(happiness.get(&(a, b)).unwrap() + happiness.get(&(b, a)).unwrap())
                    }
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn part1(input: String) -> Result<isize> {
    Ok(compute_total_happiness(parse_happiness(&input)?))
}

fn part2(input: String) -> Result<isize> {
    let (mut people, mut happiness) = parse_happiness(&input)?;
    for person in people.iter() {
        happiness.insert(("", person), 0);
        happiness.insert((person, ""), 0);
    }
    people.insert("");
    Ok(compute_total_happiness((people, happiness)))
}

#[test]
fn day13_test() {
    const EXAMPLE: &'static str = r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    assert_eq!(
        (
            ["Alice", "Bob", "Carol", "David"]
                .iter()
                .cloned()
                .collect::<HashSet<_>>(),
            [
                (("Alice", "Bob"), 54),
                (("Alice", "Carol"), -79),
                (("Alice", "David"), -2),
                (("Bob", "Alice"), 83),
                (("Bob", "Carol"), -7),
                (("Bob", "David"), -63),
                (("Carol", "Alice"), -62),
                (("Carol", "Bob"), 60),
                (("Carol", "David"), 55),
                (("David", "Alice"), 46),
                (("David", "Bob"), -7),
                (("David", "Carol"), 41),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>()
        ),
        parse_happiness(EXAMPLE).unwrap()
    );

    assert_results!(part1,
        EXAMPLE => 330,
    );
}
