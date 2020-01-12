use std::io::stdin;
use std::process::exit;
use std::env;

mod parser;
use parser::parse;

mod errs;

const MAX_PROG_LEN: usize = 1000000;

fn read_prog(mut args: env::Args) -> Result<String, String> {
    args.next();
    match args.next() {
        //TODO: add reading from file
        Some(arg) => return Ok(arg.trim().to_string()),
        None => {
            let mut inp = String::with_capacity(MAX_PROG_LEN);
            match stdin().read_line(&mut inp) {
                Ok(_) => Ok(inp), //println!("{:?} bytes read", n),
                Err(e) => panic!("WTF??? {}", e)
            }
        }
    }

}

// TODO:
// add error cursor
// read from file
// debug!

fn main() {
    let program = match read_prog(env::args()) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("Program reading error:\n---> {}", e);
            exit(1);
        }
    };

    if let Err(e) = parse(program) {
        eprintln!("{}", e);
        exit(1);
    }
}
