# Advent of Code 2015

My solutions for the Advent of Code 2015, written in Rust.

Automatically downloads puzzle input from your AoC token (stored in `token.txt`),
and caches the downloaded inputs in `cache.dat`.

# Usage

1. Clone
2. Create `token.txt` with your AoC session token (login and check the cookies)
3. `cargo run --release` (optionally append ` -- dayXX` to run a specific day)