day!(
    day20,
    "https://adventofcode.com/2015/day/20/input",
    part1,
    part2
);

use itertools::Itertools;
use std::iter;

lazy_static! {
    static ref SIEVE: primal_sieve::Sieve = primal_sieve::Sieve::new(100_000_000);
}

/// Iterates over all the positive integer divisors of a number (by doing
/// prime factorization and combinations). It then applies function f over
/// an accumulated value (starting at 0) for each divisor.
fn fold_divisors<F>(house_number: usize, f: F) -> u64
where
    F: Fn(u64, u64) -> u64,
{
    let factors = SIEVE.factor(house_number).unwrap();

    fn accumulate<F>(
        sum: &mut u64,
        factors: &Vec<(usize, usize)>,
        factor_idx: usize,
        multiplier: u64,
        f: &F,
    ) where
        F: Fn(u64, u64) -> u64,
    {
        if factor_idx >= factors.len() {
            *sum = f(*sum, multiplier);
            return;
        }
        let (factor, exponent) = factors[factor_idx];
        let factor = factor as u64;
        let mut extra_multiplier = 1;
        for _ in 0..=exponent {
            accumulate(
                sum,
                factors,
                factor_idx + 1,
                multiplier * extra_multiplier,
                f,
            );
            extra_multiplier *= factor;
        }
    }

    let mut sum = 0;
    accumulate(&mut sum, &factors, 0, 1, &f);
    sum
}


fn part1(input: String) -> Result<usize> {
    let input: u64 = input.parse()?;
    for house_number in 1.. {
        // The amount of presents is the sum of all the positive integer divisors.
        // Multiplied by 10.
        if fold_divisors(house_number, |acc, div| acc + div) * 10 >= input {
            return Ok(house_number);
        }
    }
    unreachable!()
}

fn part2(input: String) -> Result<u64> {
    let input: u64 = input.parse()?;

    for house_number in 1u64.. {
        // The amount of presents is the sum of all the positive integer divisors
        // where those divisors (the elves) haven't previously visited 50 house
        // numbers. Multiplied by 11.
        if fold_divisors(house_number as usize, |acc, div| {
            if (house_number + div - 1) / div <= 50 {
                acc + div
            } else {
                acc
            }
        }) * 11
            >= input
        {
            return Ok(house_number);
        }
    }

    unreachable!()
}

#[test]
fn day20_test() {
    use std::ops::Add;
    assert_eq!(fold_divisors(1, Add::add), 1);
    assert_eq!(fold_divisors(2, Add::add), 3);
    assert_eq!(fold_divisors(3, Add::add), 4);
    assert_eq!(fold_divisors(4, Add::add), 7);
    assert_eq!(fold_divisors(5, Add::add), 6);
    assert_eq!(fold_divisors(6, Add::add), 12);
    assert_eq!(fold_divisors(7, Add::add), 8);
    assert_eq!(fold_divisors(8, Add::add), 15);
    assert_eq!(fold_divisors(9, Add::add), 13);
}
