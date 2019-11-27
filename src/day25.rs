day!(
    day25,
    "https://adventofcode.com/2015/day/25/input",
    part1,
    !
);

fn get_grid_index(row: u64, column: u64) -> u64 {
    (row + column + 1) * (row + column) / 2 + column
}

fn part1(input: String) -> Result<u64> {
    let (row, column) = parse_input(input)?;
    let (row, column) = (row - 1, column - 1); // Make 0 based

    let idx = get_grid_index(row, column) + 1;
    let mut nr = 20151125u64;
    for _ in 1..idx {
        nr = (nr * 252533) % 33554393;
    }

    Ok(nr)
}

fn parse_input(input: String) -> Result<(u64, u64)> {
    const PREFIX: &'static str = "To continue, please consult the code grid in the manual.  Enter the code at row ";
    const MID: &'static str = ", column ";
    const SUFFIX: &'static str = ".";

    if input.len() < PREFIX.len() + MID.len() + SUFFIX.len() + 2 {
        return Err(Error::Input("input too short"));
    }

    if &input[0..PREFIX.len()] != PREFIX {
        return Err(Error::Input("invalid prefix"));
    }
    let input = &input[PREFIX.len()..];
    let idx = input.find(MID).ok_or(Error::Input("no mid found"))?;
    let row = input[0..idx].parse()?;
    let input = &input[idx + MID.len()..];
    let idx = input.find(SUFFIX).ok_or(Error::Input("no suffix found"))?;
    let column = input[0..idx].parse()?;
    let input = &input[idx + SUFFIX.len()..];
    if input.len() != 0 {
        return Err(Error::Input("input not empty"));
    }

    Ok((row, column))
}