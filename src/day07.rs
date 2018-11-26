day!(
    day07,
    "https://adventofcode.com/2015/day/7/input",
    part1,
    part2
);

use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

type Value = u16;
type Ident = String;

#[derive(Debug, Clone)]
struct Instruction {
    target: Ident,
    action: Action,
}

#[derive(Debug, Clone)]
enum Action {
    Set(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Value),
    RShift(Operand, Value),
}

#[derive(Debug, Clone)]
enum Operand {
    Value(Value),
    Ident(Ident),
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> ::std::result::Result<Operand, ()> {
        Ok(if let Ok(nr) = s.parse() {
            Operand::Value(nr)
        } else {
            Operand::Ident(s.to_string())
        })
    }
}

fn transform(input: String) -> Vec<Instruction> {
    use self::Action::*;
    input
        .split("\n")
        .filter_map(|line| {
            let pivot = line.find(" -> ")?;
            let target = line.get(pivot + 4..)?.to_string();
            let action = line.get(0..pivot)?;
            let action = if action.starts_with("NOT ") {
                Not(action.get(4..)?.parse().ok()?)
            } else if let None = action.find(" ") {
                Set(action.parse().ok()?)
            } else {
                let mut iter = action.split(" ");
                let ident = iter.next()?.to_string();
                match iter.next()? {
                    "AND" => And(ident.parse().ok()?, iter.next()?.parse().ok()?),
                    "OR" => Or(ident.parse().ok()?, iter.next()?.parse().ok()?),
                    "LSHIFT" => LShift(ident.parse().ok()?, iter.next()?.parse().ok()?),
                    "RSHIFT" => RShift(ident.parse().ok()?, iter.next()?.parse().ok()?),
                    _ => return None,
                }
            };

            Some(Instruction { target, action })
        })
        .collect()
}

fn part1(instructions: String) -> Result<String> {
    let ref instructions = transform(instructions);
    type Instructions<'a> = HashMap<&'a str, &'a Instruction>;
    type Values<'a> = HashMap<&'a str, Value>;

    let instructions: Instructions = instructions
        .iter()
        .map(|x| (x.target.as_str(), x))
        .collect();

    let mut values: Values = HashMap::new();

    fn value_of<'a>(
        ident: &'a str,
        instructions: &Instructions<'a>,
        values: &mut Values<'a>,
    ) -> Option<Value> {
        if let Some(&value) = values.get(ident) {
            return Some(value);
        }

        let instruction = instructions.get(ident)?;
        macro_rules! resolve {
            ($ident:expr) => {
                match $ident {
                    Operand::Value(value) => *value,
                    Operand::Ident(ident) => value_of(ident.as_str(), instructions, values)?,
                }
            };
        }

        use self::Action::*;
        let value = match &instruction.action {
            Set(a) => resolve!(a),
            Not(a) => !resolve!(a),
            And(a, b) => resolve!(a) & resolve!(b),
            Or(a, b) => resolve!(a) | resolve!(b),
            LShift(a, b) => resolve!(a) << b,
            RShift(a, b) => resolve!(a) >> b,
        };

        values.insert(ident, value);
        Some(value)
    }

    for &ident in instructions.keys() {
        value_of(ident, &instructions, &mut values);
    }

    Ok(values
        .iter()
        .sorted()
        .iter()
        .map(|(name, value)| format!("{}: {}", name, value))
        .take(8)
        .join("\n"))
}

fn part2(instructions: String) -> Result<String> {
    let ref instructions = transform(instructions);

    type Instructions<'a> = HashMap<&'a str, &'a Instruction>;
    type Values<'a> = HashMap<&'a str, Value>;

    let instructions: Instructions = instructions
        .iter()
        .map(|x| (x.target.as_str(), x))
        .collect();

    let mut values: Values = HashMap::new();

    fn value_of<'a>(
        ident: &'a str,
        instructions: &Instructions<'a>,
        values: &mut Values<'a>,
    ) -> Option<Value> {
        if let Some(&value) = values.get(ident) {
            return Some(value);
        }

        let instruction = instructions.get(ident)?;
        macro_rules! resolve {
            ($ident:expr) => {
                match $ident {
                    Operand::Value(value) => *value,
                    Operand::Ident(ident) => value_of(ident.as_str(), instructions, values)?,
                }
            };
        }

        use self::Action::*;
        let value = match &instruction.action {
            Set(a) => resolve!(a),
            Not(a) => !resolve!(a),
            And(a, b) => resolve!(a) & resolve!(b),
            Or(a, b) => resolve!(a) | resolve!(b),
            LShift(a, b) => resolve!(a) << b,
            RShift(a, b) => resolve!(a) >> b,
        };

        values.insert(ident, value);
        Some(value)
    }

    for &ident in instructions.keys() {
        value_of(ident, &instructions, &mut values);
    }

    {
        let v = values[&"a"];
        values.clear();
        values.insert(&"b", v);
    }

    for &ident in instructions.keys() {
        value_of(ident, &instructions, &mut values);
    }

    Ok(values
        .iter()
        .sorted()
        .iter()
        .map(|(name, value)| format!("{}: {}", name, value))
        .take(8)
        .join("\n"))
}

#[test]
fn day07_test() {
    assert_results!(part1,
"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i" => "d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456"
    );
}
