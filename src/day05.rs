day!(
    day05,
    "https://adventofcode.com/2015/day/5/input",
    part1,
    part2
);

fn part1(input: String) -> Result<usize> {
    fn is_nice(input: &str) -> bool {
        let input = input.as_bytes();
        let vowel_count = input
            .iter()
            .filter(|x| match x {
                b'a' => true,
                b'e' => true,
                b'i' => true,
                b'o' => true,
                b'u' => true,
                _ => false,
            })
            .count();
        if vowel_count < 3 {
            return false;
        }
        let has_duplicate = input.iter().zip(input.iter().skip(1)).any(|(p, c)| p == c);
        if !has_duplicate {
            return false;
        }
        !(0..input.len() - 1)
            .map(|i| (input[i], input[i + 1]))
            .any(|substr| match substr {
                (b'a', b'b') => true,
                (b'c', b'd') => true,
                (b'p', b'q') => true,
                (b'x', b'y') => true,
                _ => false,
            })
    }

    Ok(input.split('\n').filter(|x| is_nice(x)).count())
}

fn part2(input: String) -> Result<usize> {
    fn is_nice(input: &str) -> bool {
        let input = input.as_bytes();
        if !(0..input.len() - 2)
            .map(|i| (input[i], input[i + 2]))
            .any(|(a, b)| a == b)
        {
            return false;
        }

        (0..input.len()-2)
            .any(|i| {
                let (a, b) = (input[i], input[i + 1]);
                (i + 2.. input.len() - 1)
                    .any(|j| a == input[j] && b == input[j + 1])
            })
    }

    Ok(input.split('\n').filter(|x| is_nice(x)).count())
}

#[test]
fn day05_test() {
    assert_results!(part1,
        "ugknbfddgicrmopn" => 1,
        "aaa"              => 1,
        "jchzalrnumimnmhp" => 0,
        "haegwjzuvuyypxyu" => 0,
        "dvszwmarrgswjxmb" => 0,
    );

    assert_results!(part2,
        "qjhvhtzxzqqjkmpb" => 1,
        "xxyxx"            => 1,
        "uurcxstgmygtbstg" => 0,
        "ieodomkazucvgmuy" => 0,
    );
}

