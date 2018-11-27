day!(
    day14,
    "https://adventofcode.com/2015/day/14/input",
    part1,
    part2
);

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Mobility {
    speed: usize,
    duration: usize,
    rest_time: usize,
}
type Reindeers<'a> = HashMap<&'a str, Mobility>;

fn parse_reindeers<'a>(input: &'a str) -> Result<Reindeers<'a>> {
    lazy_static!(
        static ref RE: Regex = Regex::new(r"(?m)^(?P<n>[[:alpha:]]+) can fly (?P<s>\d+) km/s for (?P<d>\d+) seconds, but then must rest for (?P<r>\d+) seconds\.$").unwrap();
    );
    RE.captures_iter(input)
        .map(|m| {
            let name = m.name("n").unwrap().as_str();
            let speed = m["s"].parse()?;
            let duration = m["d"].parse()?;
            let rest_time = m["r"].parse()?;
            Ok((
                name,
                Mobility {
                    speed,
                    duration,
                    rest_time,
                },
            ))
        })
        .collect()
}

fn compute_at_time(mobility: &Mobility, time: usize) -> usize {
    let cycle_time = mobility.duration + mobility.rest_time;
    let cycle_count = time / cycle_time;
    let remaining_time = time - cycle_count * cycle_time;
    let cycle_distance = cycle_count * (mobility.speed * mobility.duration);
    if remaining_time >= mobility.duration {
        cycle_distance + mobility.speed * mobility.duration
    } else {
        cycle_distance + mobility.speed * remaining_time
    }
}

fn part1(input: String) -> Result<usize> {
    Ok(parse_reindeers(&input)?
        .into_iter()
        .map(|(_, mobility)| compute_at_time(&mobility, 2503))
        .max()
        .unwrap())
}

fn part2_impl(input: &str, time: usize) -> Result<usize> {
    #[derive(Clone)]
    struct State {
        mobility: Mobility,
        points: usize,
        distance: usize,
    }

    let mut states = parse_reindeers(input)?
        .into_iter()
        .map(|(_, mobility)| State {
            mobility,
            points: 0,
            distance: 0,
        })
        .collect::<Vec<_>>();

    for current_time in 1..time + 1 {
        // Update all distances
        for state in &mut states {
            state.distance = compute_at_time(&state.mobility, current_time);
        }

        // Award points
        let highest = states.iter().map(|state| state.distance).max().unwrap();
        for state in &mut states {
            if state.distance == highest {
                state.points += 1;
            }
        }
    }

    Ok(states.into_iter().map(|x| x.points).max().unwrap())
}

fn part2(input: String) -> Result<usize> {
    part2_impl(&input, 2503)
}

#[test]
fn day14_test() {
    const EXAMPLE: &'static str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    assert_eq!(
        parse_reindeers(EXAMPLE).unwrap(),
        [
            (
                "Comet",
                Mobility {
                    speed: 14,
                    duration: 10,
                    rest_time: 127
                }
            ),
            (
                "Dancer",
                Mobility {
                    speed: 16,
                    duration: 11,
                    rest_time: 162
                }
            )
        ]
        .iter()
        .cloned()
        .collect()
    );

    assert_eq!(
        compute_at_time(
            &Mobility {
                speed: 14,
                duration: 10,
                rest_time: 127
            },
            1000
        ),
        1120
    );
    assert_eq!(
        compute_at_time(
            &Mobility {
                speed: 16,
                duration: 11,
                rest_time: 162
            },
            1000
        ),
        1056
    );

    assert_eq!(
        part2_impl(
            EXAMPLE,
            1000
        )
        .unwrap(),
        689
    );
}
