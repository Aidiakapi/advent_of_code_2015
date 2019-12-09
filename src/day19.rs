day!(
    day19,
    "https://adventofcode.com/2015/day/19/input",
    part1,
    part2
);

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::mem::drop;

fn parse_input<'s>(input: &'s str) -> Result<(Vec<(&'s str, &'s str)>, &'s str)> {
    let mut iter = input.lines();
    let mut rules = Vec::new();
    loop {
        let line = iter.next().ok_or(Error::Input("unexpected end of input"))?;
        if line.len() == 0 {
            break;
        }
        let mut parts = line.split(" => ");
        let from = parts.next().ok_or(Error::Input("expected input atom"))?;
        let into = parts.next().ok_or(Error::Input("expected output atom"))?;
        if let Some(_) = parts.next() {
            return Err(Error::Input("expected end of line"));
        }
        rules.push((from, into));
    }
    let molecule = iter.next().ok_or(Error::Input("unexpected end of input"))?;
    if let Some(_) = iter.next() {
        return Err(Error::Input("expected end of input"));
    }
    Ok((rules, molecule))
}

fn part1(input: String) -> Result<usize> {
    let (rules, molecule) = parse_input(&input)?;

    let mut combinations = HashSet::new();

    for (from, to) in rules {
        for i in 0..molecule.len() - from.len() + 1 {
            if &molecule[i..i + from.len()] == from {
                let mut substitution =
                    String::with_capacity(molecule.len() - from.len() + to.len());
                substitution.push_str(&molecule[0..i]);
                substitution.push_str(&to);
                substitution.push_str(&molecule[i + from.len()..]);
                combinations.insert(substitution);
            }
        }
    }

    Ok(combinations.len())
}

type Atom = u8;
type Molecule = Vec<Atom>;

/// Turns a string into a molecule, which is a more efficient in-memory representation
/// The maximum amount of unique atoms (including the electron) is 255.
fn string_to_molecule<'s>(s: &'s str, atom_map: &mut HashMap<&'s str, Atom>) -> Result<Molecule> {
    let mut molecule = Molecule::new();
    let mut idx = 0;
    let mut bytes = s.bytes().peekable();
    while let Some(byte) = bytes.next() {
        if !byte.is_ascii_alphabetic() {
            return Err(Error::Input("non-alphabetic character in molecules"));
        }
        let atom_length = if byte == b'e' {
            if s != "e" {
                return Err(Error::Input(
                    "electron isn't allowed within a complex molecule",
                ));
            }
            1
        } else {
            if !byte.is_ascii_uppercase() {
                return Err(Error::Input(
                    "expected atom to start with an uppercase character",
                ));
            }
            let mut len = 1;
            while let Some(next_char) = bytes.peek().cloned() {
                if next_char.is_ascii_lowercase() {
                    len += 1;
                    bytes.next();
                    continue;
                }
                break;
            }
            len
        };
        let slice = &s[idx..idx + atom_length];
        idx += atom_length;
        let atom_map_len = atom_map.len();
        if atom_map_len == 256 {
            return Err(Error::Input("too many unique atoms"));
        }
        let atom = *atom_map.entry(slice).or_insert(atom_map_len as Atom);
        molecule.push(atom);
    }
    Ok(molecule)
}

fn find_substr(target: &[u8], substr: &[u8]) -> Option<usize> {
    target
        .windows(substr.len())
        .position(|window| window == substr)
}

// The way part2 is made to run efficiently is a combination of
// three concepts. The first key point is that instead of working
// forward towards the molecule, the problem is reversed, and looks
// at what molecules could be turned into the desired molecule.
//
// Secondly, since the shortest amount of steps is required, it
// uses the A* algorithm, with the current molecule length as a
// heuristic for how far off it is from reaching the starting
// point of a single electron.
// 
// Finally, it simplifies the problem slightly by removing all
// the string processing from the actual part that does work, and
// instead assigns every atom a unique number that also has a one
// byte footprint.
//
// On my particular input, each molecule on average had 11.5 ways
// it could've been produced (out of the 43 maximum). A naive BFS
// would lead towards 11.5^n expansions, A* on the other hand can
// behave closer to a DFS when it's on the right track.
fn part2(input: String) -> Result<usize> {
    let (rules, molecule) = parse_input(&input)?;

    let mut atom_map = HashMap::new();
    atom_map.insert("e", 0);
    let mut rules = rules
        .into_iter()
        .map(|(from, into)| {
            let from = string_to_molecule(&from, &mut atom_map)?;
            if from.len() != 1 {
                return Err(Error::Input(
                    "mapping can only be from one atom to a molecule",
                ));
            }
            let into = string_to_molecule(&into, &mut atom_map)?;
            Ok((into, from[0]))
        })
        .collect::<Result<Vec<_>>>()?;
    rules.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
    let molecule = string_to_molecule(&molecule, &mut atom_map)?;

    let mut astar = crate::astar::AStar::new();
    astar
        .solve(
            molecule,
            |molecule| {
                let mut candidates = arrayvec::ArrayVec::<[Molecule; 32]>::new();
                for (into, from) in &rules {
                    let idx = match find_substr(molecule.as_slice(), into) {
                        Some(idx) => idx,
                        None => continue,
                    };
                    let mut new_molecule = Molecule::with_capacity(molecule.len() + 1 - into.len());
                    new_molecule.extend_from_slice(&molecule[0..idx]);
                    new_molecule.push(*from);
                    new_molecule.extend_from_slice(&molecule[idx + into.len()..]);
                    candidates.push(new_molecule);
                }
                candidates.into_iter().map(|a| (a, 1))
            },
            |molecule| molecule.len().max(1) - 1,
            |molecule| molecule.len() == 1 && molecule[0] == 0,
        )
        .map(|path| path.last().unwrap().1)
        .ok_or(Error::Input("no solution found for input"))
}

#[test]
fn day19_test() {
    assert_results!(part1, "\
H => HO
H => OH
O => HH

HOH" => 4, "\
H => HO
H => OH
O => HH

HOHOHO" => 7,
    );

    assert_results!(part2, "\
e => H
e => O
H => HO
H => OH
O => HH

HOH" => 3, "\
e => H
e => O
H => HO
H => OH
O => HH

HOHOHO" => 6
    );
}
