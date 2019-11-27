day!(
    day24,
    "https://adventofcode.com/2015/day/24/input",
    part1,
    part2
);

use itertools::Itertools;
use std::iter::{FromIterator, Iterator};

fn difference<A, B, I>(a: A, mut b: B) -> Difference<A, B, I>
where
    A: Iterator<Item = I>,
    B: Iterator<Item = I>,
    I: Clone + Eq,
{
    let n = b.next();
    Difference(a, b, n)
}

struct Difference<A, B, I>(A, B, Option<I>);

impl<A, B, I> Iterator for Difference<A, B, I>
where
    A: Iterator<Item = I>,
    B: Iterator<Item = I>,
    I: Clone + Eq,
{
    type Item = I;
    fn next(&mut self) -> Option<I> {
        loop {
            let item = self.0.next()?;
            match self.2.as_ref() {
                None => return Some(item),
                Some(other) => {
                    if other == &item {
                        self.2 = self.1.next();
                    } else {
                        return Some(item);
                    }
                }
            }
        }
    }
}

fn compute_quantum_entanglement(slice: &[u64]) -> u64 {
    slice.iter().fold(1, |acc, nr| acc * (*nr))
}

fn solve(input: String, buckets: u64) -> Result<u64> {
    let mut nrs = input
        .lines()
        .map(|nr| Ok(nr.parse()?))
        .collect::<Result<Vec<u64>>>()?;
    nrs.sort_unstable_by(|a, b| b.cmp(&a));

    let total_weight = nrs.iter().sum::<u64>();
    if total_weight % buckets != 0 {
        return Err(Error::Input("sum of the numbers has to be a multiple of 3"));
    }

    let mut combinations = Vec::new();
    let mut combinations_other_two = Vec::new();
    let mut backtrack_stack = Vec::new();
    let mut remainder = Vec::new();

    let weight_per_bucket = total_weight / buckets;
    for number_count in 1..(nrs.len() + buckets as usize - 1) / buckets as usize {
        if nrs[0..number_count].iter().sum::<u64>() < weight_per_bucket {
            continue;
        }

        combinations.clear();
        fn backtrack(
            stack: &mut Vec<u64>,
            combinations: &mut Vec<Vec<u64>>,
            nrs: &Vec<u64>,
            weight_per_bucket: u64,
            number_count: usize,
            start_idx: usize,
        ) {
            if stack.len() == number_count {
                if stack.iter().sum::<u64>() == weight_per_bucket {
                    combinations.push(Vec::from_iter(stack.iter().cloned()));
                }
                return;
            }
            for i in start_idx..nrs.len() {
                stack.push(nrs[i]);
                backtrack(
                    stack,
                    combinations,
                    nrs,
                    weight_per_bucket,
                    number_count,
                    i + 1,
                );
                stack.pop();
            }
        }
        backtrack(
            &mut backtrack_stack,
            &mut combinations,
            &nrs,
            weight_per_bucket,
            number_count,
            0,
        );
        assert!(backtrack_stack.is_empty());
        if combinations.len() == 0 {
            continue;
        }

        combinations.sort_unstable_by(|a, b| {
            compute_quantum_entanglement(a).cmp(&compute_quantum_entanglement(b))
        });

        for combination in &mut combinations {
            combination.sort_unstable_by(|a, b| b.cmp(&a));
            remainder.clear();
            remainder.extend(difference(nrs.iter().cloned(), combination.iter().cloned()));

            let mut is_valid = false;
            for j in 1..(remainder.len() + buckets as usize - 2) / (buckets as usize - 1) {
                combinations_other_two.clear();
                backtrack(
                    &mut backtrack_stack,
                    &mut combinations_other_two,
                    &remainder,
                    weight_per_bucket,
                    j,
                    0,
                );
                if combinations_other_two.len() != 0 {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                continue;
            }
            return Ok(compute_quantum_entanglement(&combination));
        }
    }

    return Err(Error::Input("no solution found"));
}

fn part1(input: String) -> Result<u64> {
    solve(input, 3)
}
fn part2(input: String) -> Result<u64> {
    solve(input, 4)
}

#[test]
fn day24_test() {
    assert_results!(part1, "\
1
2
3
4
5
7
8
9
10
11" => 99);
}
