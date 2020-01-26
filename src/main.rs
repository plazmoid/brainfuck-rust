use std::{
    env,
    fs::File,
    io::{prelude::*, stdin},
    process::exit,
};

mod errors;
mod parser;
use errors::{BFParseError::*, BFError};

const MAX_PROG_LEN: usize = 1000000;

fn read_prog(mut args: env::Args) -> Result<String, BFError> {
    match args.nth(1) {
        Some(mut arg) => {
            let mut result = String::new();
            arg = arg.trim().to_string();
            match File::open(&arg) {
                Ok(mut f) => {
                    if let Err(_) = f.read_to_string(&mut result) {
                        return Err(BFError::new(IoReadingErr, None, None));
                    };
                    return Ok(result);
                }
                Err(e) => return Err(BFError::new(IoReadingErr, None, Some(e.to_string()))),
            }
        }
        None => {
            let mut inp = String::with_capacity(MAX_PROG_LEN);
            match stdin().read_line(&mut inp) {
                Ok(_) => return Ok(inp),
                Err(e) => panic!("WTF??? {}", e),
            }
        }
    }
}

// TODO:
// add error cursor
//// break source by 30 chars on line and highlight

fn main() {
    let program = read_prog(env::args()).unwrap_or_else(|e| {
        eprintln!("Program loading error: \n{:?}", e);
        exit(1);
    });

    parser::parse(program).unwrap_or_else(|e| {
        eprintln!("Error: \n{:?}", e);
        exit(1);
    });
}
