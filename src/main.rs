mod common_handler;
mod def_handler;
mod lef_handler;

use lalrpop_util::lalrpop_mod;

use eyre::{eyre, Result};
use std::ffi::OsString;

use pico_args::Arguments;

use crate::{def_handler::read_def, lef_handler::read_lef};

//lalrpop_mod!(pub lef_grammar_small);
lalrpop_mod!(pub lef);
lalrpop_mod!(pub def);

const USAGE: &str = "
Usage: pascal <inputs>...

Parses each input file.

-h --help Print help.
";

#[derive(Debug)]
struct Args {
    arg_inputs: Vec<OsString>,
    flag_help: bool,
}

fn parse_args(mut args: Arguments) -> Result<Args, Box<dyn std::error::Error>> {
    Ok(Args {
        flag_help: args.contains(["-h", "--help"]),
        arg_inputs: args.finish(),
    })
}

fn main() -> Result<()> {
    let args = parse_args(Arguments::from_env()).unwrap();

    if args.flag_help {
        return Err(eyre!("{}", USAGE));
    }

    for input in &args.arg_inputs {
        if input
            .to_str()
            .expect("Input is corrupted")
            .ends_with(".def")
        {
            let def = read_def(input)?;
            println!("{}", def);
        }

        if input
            .to_str()
            .expect("Input is corrupted")
            .ends_with(".lef")
        {
            let lef = read_lef(input)?;
            println!("{}", lef);
        }
    }

    Ok(())
}
