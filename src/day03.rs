day!(
    day03,
    "https://adventofcode.com/2015/day/3/input",
    part1,
    part2
);

use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    West,
    East,
    South,
    North,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        use self::Direction::*;
        match *self {
            West => (-1, 0),
            East => (1, 0),
            South => (0, -1),
            North => (0, 1),
        }
    }

    fn apply(&self, (x, y): &mut (isize, isize)) {
        let (ox, oy) = self.offset();
        *x += ox;
        *y += oy;
    }
}

fn transform(input: String) -> Vec<Direction> {
    use self::Direction::*;
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(West),
            '>' => Some(East),
            'v' => Some(South),
            '^' => Some(North),
            _ => None,
        })
        .collect()
}

fn part1(input: String) -> Result<usize> {
    let input = transform(input);
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(pos);
    for direction in &input {
        direction.apply(&mut pos);
        visited.insert(pos);
    }
    Ok(visited.len())
}

fn part2(input: String) -> Result<usize> {
    let input = transform(input);
    let (mut posa, mut posb) = ((0, 0), (0, 0));
    let mut visited = HashSet::new();
    visited.insert(posa);
    for direction in &input {
        direction.apply(&mut posa);
        visited.insert(posa);
        ::std::mem::swap(&mut posa, &mut posb);
    }
    Ok(visited.len())
}

#[test]
fn day03_test() {
    assert_results!(part1,
        ">"          => 2,
        "^>v<"       => 4,
        "^v^v^v^v^v" => 2,
    );
    assert_results!(part2,
        "^v"         => 3,
        "^>v<"       => 3,
        "^v^v^v^v^v" => 11,
    );
}
