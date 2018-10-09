#![feature(associated_type_defaults, box_syntax)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate reqwest;
extern crate itertools;
extern crate crypto;

#[macro_use]
mod framework;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let mut fw = framework::Framework::new();
    macro_rules! load_days {
        ($($e: ident),+) => {
            $(
                fw.set_active_module(stringify!($e));
                $e::load(&mut fw);
            )*
        };
    }

    load_days!(
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08
    );

    match fw.execute() {
        Ok(()) => println!("All OK"),
        Err(x) => eprintln!("IO error:\n{:?}", x),
    }
}
