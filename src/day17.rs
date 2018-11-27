day!(
    day17,
    "https://adventofcode.com/2015/day/17/input",
    part1,
    part2
);

use std::cmp::Ordering;
use std::collections::HashMap;

fn combinations(input: &str, total: usize) -> Result<usize> {
    let mut sizes: Vec<usize> = input.lines().map(|x| Ok(x.parse()?)).collect::<Result<_>>()?;
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let mut combinations = 0;

    fn visit(total: usize, idx: usize, sizes: &Vec<usize>, combinations: &mut usize, previous: usize) {
        let current = sizes[idx];
        match (previous + current).cmp(&total) {
            Ordering::Less => {
                for i in idx+1..sizes.len() {
                    visit(total, i, sizes, combinations, previous + current);
                }
            },
            Ordering::Equal => {
                *combinations += 1;
            }
            Ordering::Greater => {
            }
        }
    }
    let upper = {
        let mut sum = 0;
        let mut i = sizes.len();
        loop {
            sum += sizes[i - 1];
            if sum >= total {
                break i;
            }
            i -= 1;
        }
    };

    for i in 0..upper {
        visit(total, i, &mut sizes, &mut combinations, 0);
    }

    Ok(combinations)
}

fn part1(input: String) -> Result<usize> {
    combinations(&input, 150)
}

fn minimum_combinations(input: &str, total: usize) -> Result<usize> {
    let mut sizes: Vec<usize> = input.lines().map(|x| Ok(x.parse()?)).collect::<Result<_>>()?;
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let mut combinations = HashMap::new();

    fn visit(total: usize, idx: usize, sizes: &Vec<usize>, combinations: &mut HashMap<usize, usize>, previous: usize, count: usize) {
        let current = sizes[idx];
        match (previous + current).cmp(&total) {
            Ordering::Less => {
                for i in idx+1..sizes.len() {
                    visit(total, i, sizes, combinations, previous + current, count + 1);
                }
            },
            Ordering::Equal => {
                *combinations.entry(count + 1).or_insert(0) += 1;
            }
            Ordering::Greater => {
            }
        }
    }
    let upper = {
        let mut sum = 0;
        let mut i = sizes.len();
        loop {
            sum += sizes[i - 1];
            if sum >= total {
                break i;
            }
            i -= 1;
        }
    };

    for i in 0..upper {
        visit(total, i, &mut sizes, &mut combinations, 0, 0);
    }

    Ok(combinations.into_iter().min_by_key(|v| v.0).unwrap().1)
}

fn part2(input: String) -> Result<usize> {
    minimum_combinations(&input, 150)
}

#[test]
fn day17_test() {
    assert_eq!(combinations("20\n15\n10\n5\n5", 25).unwrap(), 4);
    assert_eq!(minimum_combinations("20\n15\n10\n5\n5", 25).unwrap(), 3);
}
