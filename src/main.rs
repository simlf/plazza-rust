#![allow(unused)]

// Extern crate
use clap::Parser;
use anyhow::Result;
use std::{io::Write, collections::VecDeque};

// Local
pub mod pizza;
pub mod shell;
use crate::shell::Shell;

#[derive(Parser)]
struct Args {
    cooking_time: f64,
    nb_cooks: f64,
    ingredient_replacement_time: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.cooking_time < 0.0 || args.cooking_time > 1.0 {
        panic!("Cooking time should be between 0 and 1.");
    }
    let mut shell : Shell = Shell {
        pizzas: VecDeque::new()
    };
    shell.run();
    Ok(())
}

