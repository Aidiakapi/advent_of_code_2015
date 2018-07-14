use framework::*;

pub fn load(fw: &mut Framework) {
    fw.register("https://adventofcode.com/2015/day/1/input", &solution);
}

fn solution(input: String) -> Result<isize> {
    Ok(input.chars().fold(0, |a, c| match c {
        '(' => a + 1,
        ')' => a - 1,
        _ => a,
    }))
}
