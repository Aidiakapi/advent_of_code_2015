day!(
    day12,
    "https://adventofcode.com/2015/day/12/input",
    part1,
    part2
);

use itertools::Itertools;

fn extract_numbers<'a>(input: &'a str) -> impl Iterator<Item = isize> + 'a {
    input.chars().peekable().batching(|it| loop {
        match it.next() {
            Some(x) if x == '-' || (x >= '0' && x <= '9') => {
                let is_negative = if x == '-' {
                    if let Some(&x) = it.peek() {
                        if x < '0' || x > '9' {
                            continue;
                        }
                    }
                    true
                } else {
                    false
                };

                let mut nr = if is_negative {
                    0
                } else {
                    x as isize - '0' as isize
                };
                while let Some(x) = it.next() {
                    if x < '0' || x > '9' {
                        break;
                    }
                    nr = nr * 10 + (x as isize) - ('0' as isize);
                }
                return Some(if is_negative { -nr } else { nr });
            }
            Some(_) => {}
            None => break None,
        }
    })
}

fn part1(input: String) -> Result<isize> {
    Ok(extract_numbers(&input).sum())
}

fn part2(input: String) -> Result<isize> {
    use serde_json::Value;
    let mut json = serde_json::from_str(input.as_str())?;
    fn prune_red(json: &mut Value) {
        match json {
            Value::Object(map) => {
                for (_, value) in map.iter_mut() {
                    if let Some(s) = value.as_str() {
                        if s == "red" {
                            map.clear();
                            return;
                        }
                    }
                }
                for (_, value) in map.iter_mut() {
                    prune_red(value);
                }
            }
            Value::Array(array) => {
                for value in array.iter_mut() {
                    prune_red(value)
                }
            }
            _ => {}
        }
    }

    prune_red(&mut json);
    part1(serde_json::to_string(&json)?)
}

#[test]
fn day12_test() {
    fn extract(input: String) -> Result<&'static [isize]> {
        let res: Box<Vec<isize>> = Box::new(extract_numbers(&input).collect());
        Ok(Box::leak(res).as_slice())
    }
    assert_results!(extract,
        r#"[1,2,3]"#              => &[1, 2, 3],
        r#"{"a":2,"b":4}"#        => &[2, 4],
        r#"[[[3]]]"#              => &[3],
        r#"{"a":{"b":4},"c":-1}"# => &[4, -1],
        r#"{"a":[-1,1]}"#         => &[-1, 1],
        r#"[-1,{"a":1}]"#         => &[-1, 1],
        r#"[]"#                   => &([] as [isize; 0]),
        r#"{}"#                   => &([] as [isize; 0]),
    );

    assert_results!(part2,
        r#"[1,2,3]"#                         => 6,
        r#"[1,{"c":"red","b":2},3]"#         => 4,
        r#"{"d":"red","e":[1,2,3,4],"f":5}"# => 0,
        r#"[1,"red",5]"#                     => 6,
    );
}
