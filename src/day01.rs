use framework::*;

pub fn load(fw: &mut Framework) {
    register!(fw, "https://adventofcode.com/2015/day/1/input", part1,
    [
        "(())"    => "0"
        "()()"    => "0"
        "((("     => "3"
        "(()(()(" => "3"
        "))(((((" => "3"
        "())"     => "-1"
        "))("     => "-1"
        ")))"     => "-3"
        ")())())" => "-3"
    ]);
    register!(fw, "https://adventofcode.com/2015/day/1/input", part2,
    [
        ")"     => "1"
        "()())" => "5"
    ]);
}

fn part1(input: String) -> Result<isize> {
    Ok(input.chars().fold(0, |a, c| match c {
        '(' => a + 1,
        ')' => a - 1,
        _ => a,
    }))
}

fn part2(input: String) -> Result<usize> {
    let mut floor = 0;
    for (index, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' if floor == 0 => return Ok(index + 1),
            ')' => floor -= 1,
            _ => (),
        }
    }
    Err("Basement level not reached".into())
}
