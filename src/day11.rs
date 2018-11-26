day!(
    day11,
    "https://adventofcode.com/2015/day/11/input",
    part1,
    part2
);

fn rule_abc(pw: &[u8]) -> bool {
    if pw.len() < 3 {
        return false;
    }
    for i in 0..pw.len() - 3 {
        if pw[i] + 1 == pw[i + 1] && pw[i] + 2 == pw[i + 2] {
            return true;
        }
    }
    false
}

fn rule_iol(pw: &[u8]) -> bool {
    pw.iter().all(|c| match *c {
        b'i' | b'o' | b'l' => false,
        _ => true,
    })
}

fn rule_two_pairs(pw: &[u8]) -> bool {
    if pw.len() < 4 {
        return false;
    }
    for i in 0..pw.len() - 3 {
        if pw[i] == pw[i + 1] {
            for j in i + 2..pw.len() - 1 {
                if pw[j] == pw[j + 1] && pw[i] != pw[j] {
                    return true;
                }
            }
            return false;
        }
    }
    return false;
}

fn cycle_password(pw: &mut [u8]) {
    fn cycle_char(idx: usize, pw: &mut [u8]) {
        if pw[idx] == b'z' {
            pw[idx] = b'a';
            cycle_char(idx - 1, pw)
        } else {
            pw[idx] = pw[idx] + 1;
        }
    }
    cycle_char(pw.len() - 1, pw);
}

fn cycle_password_until_valid(pw: &mut [u8]) {
    cycle_password(pw);
    while !rule_abc(pw) || !rule_iol(pw) || !rule_two_pairs(pw) {
        cycle_password(pw);
    }
}

fn validate_input(input: String) -> Result<Vec<u8>> {
    let pw = input.into_bytes();
    for &c in &pw {
        if c < b'a' || c > b'z' {
            return Err(Error::Puzzle("invalid input in puzzle"));
        }
    }
    Ok(pw)
}

fn part1(input: String) -> Result<String> {
    let mut pw = validate_input(input)?;
    cycle_password_until_valid(&mut pw);
    Ok(unsafe { String::from_utf8_unchecked(pw) })
}
fn part2(input: String) -> Result<String> {
    let mut pw = validate_input(input)?;
    cycle_password_until_valid(&mut pw);
    cycle_password_until_valid(&mut pw);
    Ok(unsafe { String::from_utf8_unchecked(pw) })
}

#[test]
fn day11_test() {
    assert!(rule_abc(b"hijklmmn"));
    assert!(!rule_iol(b"hijklmmn"));
    assert!(!rule_two_pairs(b"hijklmmn"));
    assert!(!rule_abc(b"abbceffg"));
    assert!(rule_iol(b"abbceffg"));
    assert!(rule_two_pairs(b"abbceffg"));
    assert!(!rule_abc(b"abbcegjk"));
    assert!(rule_iol(b"abbcegjk"));
    assert!(!rule_two_pairs(b"abbcegjk"));

    let mut pw = *b"abcdefgh";
    cycle_password_until_valid(&mut pw[..]);
    assert_eq!(b"abcdffaa", &pw);

    let mut pw = *b"ghijklmn";
    cycle_password_until_valid(&mut pw[..]);
    assert_eq!(b"ghjaabcc", &pw);
}
