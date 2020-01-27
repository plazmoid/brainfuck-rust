use std::{
    env,
    fs::File,
    io::{prelude::*, stdin},
    process::exit,
    vec::Vec
};

mod errors;
mod parser;
mod automata;
use errors::{BFParseError::*, BFError};
use parser::Reader;


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
            let mut inp = String::with_capacity(Reader::MAX_PROG_LEN);
            match stdin().read_line(&mut inp) {
                Ok(_) => return Ok(inp),
                Err(e) => panic!("WTF??? {}", e),
            }
        }
    }
}

fn main() {
    let program = read_prog(env::args()).unwrap_or_else(|e| {
        eprintln!("Program loading error: \n{:?}", e);
        exit(1);
    });

    let mut parser = Reader::new();
    parser.parse(program).unwrap_or_else(|errs: &Vec<BFError>| {
        errs.iter().for_each(|err| eprintln!("{:?}", err));
        exit(1);
    });
}
