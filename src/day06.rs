day!(
    day06,
    "https://adventofcode.com/2015/day/6/input",
    part1,
    part2
);

use std::str::FromStr;
use std::num::ParseIntError;

struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ParsePointErr {
    EndOfInput,
    RemainingInput,
    ParseInt(ParseIntError),
}
impl From<ParseIntError> for ParsePointErr {
    fn from(e: ParseIntError) -> ParsePointErr {
        ParsePointErr::ParseInt(e)
    }
}
impl FromStr for Point {
    type Err = ParsePointErr;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let mut iter = s.split(",");

        let x = iter.next().ok_or(ParsePointErr::EndOfInput)?.parse()?;
        let y = iter.next().ok_or(ParsePointErr::EndOfInput)?.parse()?;
        if let Some(_) = iter.next() {
            return Err(ParsePointErr::RemainingInput);
        }

        Ok(Point { x, y })
    }
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    action: Action,
    from: Point,
    to: Point,
}

fn transform(input: String) -> Vec<Instruction> {
    use self::Action::*;
    input
        .split("\n")
        .filter_map(|line| {
            let (action, remainder) = if line.starts_with("turn on ") {
                (TurnOn, &line[8..])
            } else if line.starts_with("turn off ") {
                (TurnOff, &line[9..])
            } else if line.starts_with("toggle ") {
                (Toggle, &line[7..])
            } else {
                return None;
            };
            let mut iter = remainder.split(' ');
            let from = iter.next()?;
            if iter.next()? != "through" {
                return None;
            }
            let to = iter.next()?;
            if let Some(_) = iter.next() {
                return None;
            }
            Some(Instruction {
                action,
                from: from.parse().ok()?,
                to: to.parse().ok()?,
            })
        })
        .collect()
}
fn part1(instructions: String) -> Result<usize> {
    let instructions = transform(instructions);
    let mut grid = vec![[false; 1000]; 1000];
    for instruction in instructions {
        for x in instruction.from.x..(instruction.to.x + 1) {
            for y in instruction.from.y..(instruction.to.y + 1) {
                use self::Action::*;
                match instruction.action {
                    TurnOn => grid[x][y] = true,
                    TurnOff => grid[x][y] = false,
                    Toggle => grid[x][y] = !grid[x][y],
                };
            }
        }
    }
    Ok(grid.iter().flat_map(|x| x.iter()).filter(|&&x| x).count())
}

fn part2(instructions: String) -> Result<usize> {
    let instructions = transform(instructions);
    let mut grid = vec![[0; 1000]; 1000];
    for instruction in instructions {
        for x in instruction.from.x..(instruction.to.x + 1) {
            for y in instruction.from.y..(instruction.to.y + 1) {
                use self::Action::*;
                match instruction.action {
                    TurnOn => grid[x][y] = grid[x][y] + 1,
                    TurnOff if grid[x][y] > 0 => grid[x][y] = grid[x][y] - 1,
                    TurnOff => (),
                    Toggle => grid[x][y] = grid[x][y] + 2,
                };
            }
        }
    }
    Ok(grid.iter().flat_map(|x| x.iter()).sum())
}

#[test]
fn day06_test() {
    assert_results!(part1,
        "turn on 0,0 through 999,999" => 1000000,
        "toggle 0,0 through 999,0"    => 1000,
        "turn on 0,0 through 999,999\nturn off 499,499 through 500,500" => 999996,
    );

    assert_results!(part2,
        "turn on 0,0 through 0,0"    => 1,
        "toggle 0,0 through 999,999" => 2000000,
    );
}
