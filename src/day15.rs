day!(
    day15,
    "https://adventofcode.com/2015/day/15/input",
    part1,
    part2
);

use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse_ingredients(input: &str) -> Result<Vec<Ingredient>> {
    let re = Regex::new(r"(?m)^(?P<n>[[:alpha:]]+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)$").unwrap();
    re.captures_iter(input)
        .map(|m| {
            Ok(Ingredient {
                capacity: m["capacity"].parse()?,
                durability: m["durability"].parse()?,
                flavor: m["flavor"].parse()?,
                texture: m["texture"].parse()?,
                calories: m["calories"].parse()?,
            })
        })
        .collect()
}

fn for_each_combination<F>(ingredients: &Vec<Ingredient>, mut f: F) -> Result<()>
where
    F: FnMut(isize, isize, isize, isize) -> ()
{
    if ingredients.len() != 4 {
        return Err(Error::Input("can only handle 4 ingredients"));
    }
    
    for a in 0..101 {
        let remainder = 100 - a;
        for b in 0..remainder + 1 {
            let remainder = remainder - b;
            for c in 0..remainder + 1 {
                let remainder = remainder - c;
                let d = remainder;
                f(a, b, c, d);
            }
        }
    }
    Ok(())
}

fn part1(input: String) -> Result<isize> {
    let ingredients = parse_ingredients(&input)?;
    let mut max = 0;
    for_each_combination(&ingredients, |a, b, c, d| #[rustfmt::skip] {
        let capacity   = isize::max(0, a * ingredients[0].capacity   + b * ingredients[1].capacity   + c * ingredients[2].capacity   + d * ingredients[3].capacity  );
        let durability = isize::max(0, a * ingredients[0].durability + b * ingredients[1].durability + c * ingredients[2].durability + d * ingredients[3].durability);
        let flavor     = isize::max(0, a * ingredients[0].flavor     + b * ingredients[1].flavor     + c * ingredients[2].flavor     + d * ingredients[3].flavor    );
        let texture    = isize::max(0, a * ingredients[0].texture    + b * ingredients[1].texture    + c * ingredients[2].texture    + d * ingredients[3].texture   );
        // let calories   = isize::max(0, a * ingredients[0].calories   + b * ingredients[1].calories   + c * ingredients[2].calories   + d * ingredients[3].calories  );
        let value = capacity * durability * flavor * texture;
        if value > max { max = value; }
    })?;
    Ok(max)
}

fn part2(input: String) -> Result<isize> {
    let ingredients = parse_ingredients(&input)?;
    let mut max = 0;
    for_each_combination(&ingredients, |a, b, c, d| #[rustfmt::skip] {
        let capacity   = isize::max(0, a * ingredients[0].capacity   + b * ingredients[1].capacity   + c * ingredients[2].capacity   + d * ingredients[3].capacity  );
        let durability = isize::max(0, a * ingredients[0].durability + b * ingredients[1].durability + c * ingredients[2].durability + d * ingredients[3].durability);
        let flavor     = isize::max(0, a * ingredients[0].flavor     + b * ingredients[1].flavor     + c * ingredients[2].flavor     + d * ingredients[3].flavor    );
        let texture    = isize::max(0, a * ingredients[0].texture    + b * ingredients[1].texture    + c * ingredients[2].texture    + d * ingredients[3].texture   );
        let calories   = isize::max(0, a * ingredients[0].calories   + b * ingredients[1].calories   + c * ingredients[2].calories   + d * ingredients[3].calories  );
        let value = capacity * durability * flavor * texture;
        if calories == 500 && value > max { max = value; }
    })?;
    Ok(max)
}

#[test]
fn day15_test() {
    const EXAMPLE: &'static str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    assert_eq!(
        vec![
            Ingredient {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            }
        ],
        parse_ingredients(EXAMPLE).unwrap()
    );
}
