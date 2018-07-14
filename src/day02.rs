use framework::*;

pub fn load(fw: &mut Framework) {
    register!(fw, "https://adventofcode.com/2015/day/2/input", transform, part1, 
    [
        "2x3x4"  => "58"
        "1x1x10" => "43"
    ]);
    register!(fw, "https://adventofcode.com/2015/day/2/input", transform, part2, 
    [
        "2x3x4"  => "34"
        "1x1x10" => "14"
    ]);
}

#[derive(Debug)]
struct Dimension(usize, usize, usize);

fn transform(input: String) -> Vec<Dimension> {
    input
        .split("\n")
        .filter_map(|part| {
            let mut iter = part.split("x");
            let l = iter.next()?.parse().ok()?;
            let w = iter.next()?.parse().ok()?;
            let h = iter.next()?.parse().ok()?;
            Some(Dimension(l, w, h))
        })
        .collect()
}

fn part1(input: Vec<Dimension>) -> Result<usize> {
    println!("{:?}", input);
    Ok(input
        .iter()
        .map(|Dimension(l, w, h)| {
            let a = l * w;
            let b = w * h;
            let c = h * l;
            let min = **(&[a, b, c].iter().min().unwrap());
            2 * (a + b + c) + min
        })
        .sum())
}

fn part2(input: Vec<Dimension>) -> Result<usize> {
    Ok(input
        .iter()
        .map(|Dimension(l, w, h)| {
            let a = l + w;
            let b = w + h;
            let c = h + l;
            let min = **(&[a, b, c].iter().min().unwrap());
            2 * min + l * w * h
        })
        .sum())
}