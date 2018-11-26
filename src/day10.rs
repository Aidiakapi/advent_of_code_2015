day!(
    day10,
    "https://adventofcode.com/2015/day/10/input",
    part1,
    part2
);

fn apply_n(input: String, times: usize) -> Result<usize> {
    let mut s = input;
    for _ in 0..times {
        s = look_and_say(&s)?;
    }
    Ok(s.len())
}

fn part1(input: String) -> Result<usize> {
    apply_n(input, 40)
}
fn part2(input: String) -> Result<usize> {
    apply_n(input, 50)
}

fn look_and_say(input: &str) -> Result<String> {
    use std::fmt::Write;
    let mut result = String::new();
    let mut chars = input.chars();
    let mut curr = chars
        .next()
        .ok_or(Error::Puzzle("look_and_say requires at least 1 character"))?;
    let mut count = 1;
    while let Some(next) = chars.next() {
        if next == curr {
            count += 1;
        } else {
            write!(result, "{}{}", count, curr)?;
            curr = next;
            count = 1;
        }
    }
    write!(result, "{}{}", count, curr)?;
    Ok(result)
}

#[test]
fn day10_test() {
    assert_eq!("11", look_and_say("1").unwrap());
    assert_eq!("21", look_and_say("11").unwrap());
    assert_eq!("1211", look_and_say("21").unwrap());
    assert_eq!("111221", look_and_say("1211").unwrap());
    assert_eq!("312211", look_and_say("111221").unwrap());
}
