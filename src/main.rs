#![allow(unused_imports)]
#![feature(trait_alias, stmt_expr_attributes)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod astar;
mod error;
#[macro_use]
mod framework;

use colored::Colorize;
use crate::framework::Framework;
use reqwest::Client;
use std::env;

pub(crate) use crate::error::Error;
pub(crate) use crate::error::Result;

macro_rules! main {
    ($($days:ident),+$(,)*) => {
        $(
            mod $days;
        )+
        fn main() {
            if cfg!(windows) {
                let _ = colored::control::set_virtual_terminal(true);
            }

            let mut fw = Framework::new();

            $(
                {
                    crate::$days::register_day(&mut fw);
                }
            )+;

            let client = Client::new();

            let args: Vec<String> = env::args().collect();
            match args.len() {
                1 => {
                    // execute all
                    $(
                        {
                            if let Err(e) = fw.execute(&client, stringify!($days)) {
                                eprintln!("{}", e.to_string().red());
                                std::process::exit(-2);
                            }
                        }
                    )+;
                },
                2 => {
                    // execute specific day
                    if let Err(e) = fw.execute(&client, args[1].as_str()) {
                        eprintln!("{}", e.to_string().bright_red());
                        std::process::exit(-2);
                    }
                },
                _ => {
                    eprintln!("too many arguments");
                    std::process::exit(-1);
                }
            }
        }
    };
}

#[rustfmt::skip] main!(
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    day24,
);
