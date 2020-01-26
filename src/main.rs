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
    args.next();
    match args.next() {
        Some(mut arg) => {
            let mut result = String::new();
            arg = arg.trim().to_string();
            match File::open(&arg) {
                Ok(mut f) => {
                    if let Err(_) = f.read_to_string(&mut result) {
                        return Err(BFError::new(IoReadingErr, 0, None));
                    };
                    return Ok(result);
                }
                Err(e) => return Err(BFError::new(IoReadingErr, 0, Some(e.to_string()))),
            }
        }
        None => {
            let mut inp = String::with_capacity(MAX_PROG_LEN);
            match stdin().read_line(&mut inp) {
                Ok(_) => Ok(inp), //println!("{:?} bytes read", n),
                Err(e) => panic!("WTF??? {}", e),
            }
        }
    }
}

// TODO:
// add error cursor
//// break source by 30 chars on line and highlight
// debug!

fn main() {
    let program = match read_prog(env::args()) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Program loading error:\n---> {:?}", e);
            exit(1);
        }
    };

    if let Err(e) = parser::parse(program) {
        eprintln!("Error: \n{:?}", e);
        exit(1);
    }
}
