day!(
    day09,
    "https://adventofcode.com/2015/day/9/input",
    part1,
    part2
);

use permutohedron::Heap;
use std::collections::HashMap;

type Place = u8;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Connection {
    a: Place,
    b: Place,
}

impl Connection {
    fn new(a: Place, b: Place) -> Connection {
        if a <= b {
            Connection { a, b }
        } else {
            Connection { a: b, b: a }
        }
    }
}

type Routes = HashMap<Connection, usize>;

fn transform(input: String) -> (Place, Routes) {
    let mut place_names = HashMap::new();
    let results = input
        .split('\n')
        .filter_map(|line| {
            let to_idx = line.find(" to ")?;
            let eq_idx = line.find(" = ")?;
            let fr = &line[0..to_idx];
            let to = &line[to_idx + 4..eq_idx];

            let l = place_names.len() as Place;
            let fr: Place = *place_names.entry(fr).or_insert(l);
            let l = place_names.len() as Place;
            let to: Place = *place_names.entry(to).or_insert(l);

            let dist: usize = line[eq_idx + 3..].parse().ok()?;
            Some((Connection::new(fr, to), dist))
        })
        .collect();
    (place_names.len() as Place, results)
}

fn route_lengths((place_count, routes): (Place, Routes)) -> Vec<usize> {
    let mut data = (0..place_count).collect::<Vec<_>>();
    let heap = Heap::new(&mut data);
    heap.map(|permutation| {
        permutation
            .iter()
            .take(permutation.len() - 1)
            .zip(permutation.iter().skip(1))
            .map(|(&from, &to)| {
                routes
                    .get(&Connection::new(from, to))
                    .expect("missing route")
            })
            .sum()
    })
    .collect()
}

fn part1(data: String) -> Result<usize> {
    let data: (Place, Routes) = transform(data);
    Ok(*route_lengths(data).iter().min().unwrap())
}

fn part2(data: String) -> Result<usize> {
    let data: (Place, Routes) = transform(data);
    Ok(*route_lengths(data).iter().max().unwrap())
}

#[test]
fn day09_test() {
    assert_results!(part1,
"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
=> 605
    );
    assert_results!(part2,
"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
=> 982
    );
}
