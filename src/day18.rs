day!(
    day18,
    "https://adventofcode.com/2015/day/18/input",
    part1,
    part2
);

use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct LightGrid {
    size: (usize, usize),
    data: Vec<bool>,
}

struct LightGridNeighbors {
    size: (usize, usize),
    idx: usize,
    base: (usize, usize),
}

impl LightGrid {
    fn neighbors(&self, position: (usize, usize)) -> LightGridNeighbors {
        LightGridNeighbors {
            size: self.size,
            idx: 0,
            base: position,
        }
    }

    fn transform_1(&self) -> LightGrid {
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, &old_state)| {
                let position = (i % self.size.0, i / self.size.0);
                let neighbors_on = self
                    .neighbors(position)
                    .filter(|&position| self[position])
                    .count();
                if old_state {
                    neighbors_on == 2 || neighbors_on == 3
                } else {
                    neighbors_on == 3
                }
            })
            .collect();
        LightGrid {
            size: self.size,
            data,
        }
    }

    fn transform_2(&self) -> LightGrid {
        let mut new = self.transform_1();
        let (w, h) = self.size;
        new[(0, 0)] = true;
        new[(w - 1, 0)] = true;
        new[(0, h - 1)] = true;
        new[(w - 1, h - 1)] = true;
        new
    }

    fn count_on(&self) -> usize {
        self.data.iter().filter(|s| **s).count()
    }
}

impl Iterator for LightGridNeighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.base;
        let (w, h) = self.size;
        loop {
            self.idx += 1;
            // 123
            // 4B5
            // 678
            #[rustfmt::skip]
            match self.idx {
                1 if x > 0     && y > 0     => return Some((x - 1, y - 1)),
                2 if              y > 0     => return Some((x    , y - 1)),
                3 if x < w - 1 && y > 0     => return Some((x + 1, y - 1)),
                4 if x > 0                  => return Some((x - 1, y    )),
                5 if x < w - 1              => return Some((x + 1, y    )),
                6 if x > 0     && y < h - 1 => return Some((x - 1, y + 1)),
                7 if              y < h - 1 => return Some((x    , y + 1)),
                8 if x < w - 1 && y < h - 1 => return Some((x + 1, y + 1)),
                1 => {}
                2 => {}
                3 => {}
                4 => {}
                5 => {}
                6 => {}
                7 => {}
                8 => {}
                _ => return None,
            }
        }
    }
}

impl Index<(usize, usize)> for LightGrid {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &bool {
        &self.data[self.size.0 * y + x]
    }
}
impl IndexMut<(usize, usize)> for LightGrid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut bool {
        &mut self.data[self.size.0 * y + x]
    }
}

impl Display for LightGrid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut res = String::with_capacity((self.size.0 + 1) * self.size.1);
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                res.push(if self[(x, y)] { '#' } else { '.' });
            }
            res.push('\n');
        }
        res.pop();
        write!(f, "{}", res)
    }
}

impl FromStr for LightGrid {
    type Err = Error;

    fn from_str(s: &str) -> Result<LightGrid> {
        let lines: Vec<_> = s.lines().collect();
        if lines.is_empty() {
            return Err(Error::Input("empty string"));
        }
        let h = lines.len();
        let w = lines[0].len();
        if !lines.iter().skip(1).all(|x| x.len() == w) {
            return Err(Error::Input("inconsistent width"));
        }
        let mut data = Vec::with_capacity(w * h);
        for line in lines {
            for c in line.chars() {
                match c {
                    '#' => data.push(true),
                    '.' => data.push(false),
                    _ => return Err(Error::Input("invalid character, expected # or .")),
                }
            }
        }
        Ok(LightGrid { size: (w, h), data })
    }
}

fn part1(input: String) -> Result<usize> {
    let mut grid: LightGrid = input.parse()?;
    for _ in 0..100 {
        grid = grid.transform_1();
    }
    Ok(grid.count_on())
}
fn part2(input: String) -> Result<usize> {
    let mut grid: LightGrid = input.parse()?;
    grid[(0, 0)] = true;
    grid[(99, 0)] = true;
    grid[(0, 99)] = true;
    grid[(99, 99)] = true;
    for _ in 0..100 {
        grid = grid.transform_2();
    }
    Ok(grid.count_on())
}

#[test]
fn day18_test() {
    const INITIAL: &'static str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
    let grid: LightGrid = INITIAL.parse().unwrap();
    assert_eq!(grid.to_string().as_str(), INITIAL);

    // Step 1
    let grid = grid.transform_1();
    assert_eq!(
        grid.to_string().as_str(),
        "..##..
..##.#
...##.
......
#.....
#.##.."
    );

    // Step 2
    let grid = grid.transform_1();
    assert_eq!(
        grid.to_string().as_str(),
        "..###.
......
..###.
......
.#....
.#...."
    );

    // Step 3
    let grid = grid.transform_1();
    assert_eq!(
        grid.to_string().as_str(),
        "...#..
......
...#..
..##..
......
......"
    );

    // Step 4
    let grid = grid.transform_1();
    assert_eq!(
        grid.to_string().as_str(),
        "......
......
..##..
..##..
......
......"
    );

    let grid: LightGrid = "##.#.#
...##.
#....#
..#...
#.#..#
####.#"
        .parse()
        .unwrap();

    // Step 1
    let grid = grid.transform_2();
    assert_eq!(
        grid.to_string().as_str(),
        "#.##.#
####.#
...##.
......
#...#.
#.####"
    );

    // Step 2
    let grid = grid.transform_2();
    assert_eq!(
        grid.to_string().as_str(),
        "#..#.#
#....#
.#.##.
...##.
.#..##
##.###"
    );

    // Step 3
    let grid = grid.transform_2();
    assert_eq!(
        grid.to_string().as_str(),
        "#...##
####.#
..##.#
......
##....
####.#"
    );

    // Step 4
    let grid = grid.transform_2();
    assert_eq!(
        grid.to_string().as_str(),
        "#.####
#....#
...#..
.##...
#.....
#.#..#"
    );

    // Step 5
    let grid = grid.transform_2();
    assert_eq!(
        grid.to_string().as_str(),
        "##.###
.##..#
.##...
.##...
#.#...
##...#"
    );
}
