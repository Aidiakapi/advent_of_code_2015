use framework::*;

pub fn load(fw: &mut Framework) {
    register!(fw, "https://adventofcode.com/2015/day/4/input", part1, 
    [
        "abcdef"  => "609043"
        "pqrstuv" => "1048970"
    ]);
    register!(fw, "https://adventofcode.com/2015/day/4/input", part2);
}

fn part1(prefix: String) -> Result<usize> {
    use crypto::{digest::Digest, md5::Md5};
    let mut hasher = Md5::new();
    let key = prefix.as_bytes();

    for i in 0.. {
        hasher.input(&key);
        hasher.input(&i.to_string().as_bytes());
        if &hasher.result_str()[0..5] == "00000" {
            return Ok(i);
        }
        hasher.reset();
    }

    unreachable!();
}
fn part2(prefix: String) -> Result<usize> {
    use crypto::{digest::Digest, md5::Md5};
    let mut hasher = Md5::new();
    let key = prefix.as_bytes();

    for i in 0.. {
        hasher.input(&key);
        hasher.input(&i.to_string().as_bytes());
        if &hasher.result_str()[0..6] == "000000" {
            return Ok(i);
        }
        hasher.reset();
    }

    unreachable!();
}
