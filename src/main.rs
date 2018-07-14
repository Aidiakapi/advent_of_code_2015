#![feature(associated_type_defaults, box_syntax)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate reqwest;
extern crate itertools;

#[macro_use]
mod framework;
mod day01;

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

    load_days!(day01);

    match fw.execute() {
        Ok(()) => println!("All OK"),
        Err(x) => eprintln!("IO error:\n{:?}", x),
    }
}
