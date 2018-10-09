use framework::{self as fw, Framework};

pub fn load(fw: &mut Framework) {
    register!(fw, "https://adventofcode.com/2015/day/8/input", part1,
    [
"\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"" => "12"
    ]);
    register!(fw, "https://adventofcode.com/2015/day/8/input", part2,
    [
"\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"" => "19"
    ]);
}

fn part1(input: String) -> fw::Result<usize> {
    Ok(input
        .split('\n')
        .filter(|line| line.len() >= 2)
        .map(|line| {
            let trimmed = &line[1..line.len() - 1];
            enum State {
                Regular,
                EscapeDetected,
                Hex01,
                Hex02(u8),
            }

            let mut state = State::Regular;
            let mut result = Vec::with_capacity(line.len() - 2);
            for byte in trimmed.bytes() {
                let new_state: State = match state {
                    State::Regular => {
                        if byte == b'\\' {
                            State::EscapeDetected
                        } else {
                            result.push(byte);
                            State::Regular
                        }
                    }
                    State::EscapeDetected => {
                        if byte == b'x' {
                            State::Hex01
                        } else if byte == b'\\' || byte == b'"' {
                            result.push(byte);
                            State::Regular
                        } else {
                            panic!("invalid character escape sequence");
                        }
                    }
                    State::Hex01 => {
                        if byte >= b'0' && byte <= b'9' {
                            State::Hex02(byte - b'0')
                        } else if byte >= b'a' && byte <= b'z' {
                            State::Hex02(byte - (b'a' - 10))
                        } else {
                            panic!("expected hex escape sequence");
                        }
                    }
                    State::Hex02(first) => {
                        let second = if byte >= b'0' && byte <= b'9' {
                            byte - b'0'
                        } else if byte >= b'a' && byte <= b'z' {
                            byte - (b'a' - 10)
                        } else {
                            panic!("expected hex escape sequence");
                        };
                        result.push((first << 4) | second);
                        State::Regular
                    }
                };
                state = new_state;
            }

            (line.to_string(), result)
        })
        .map(|(source, parsed)| source.len() - parsed.len())
        .sum())
}

fn part2(input: String) -> fw::Result<usize> {
    Ok(input
        .split('\n')
        .filter(|line| line.len() >= 2)
        .map(|line| {
            let count = line.bytes().filter(|c| *c == b'"' || *c == b'\\').count();
            count + 2
        })
        .sum())
}
