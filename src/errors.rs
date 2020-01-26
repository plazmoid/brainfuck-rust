use std::{
    fmt,
    env,
    cmp::{max, min}
};

use crate::parser::{
    PROG
};

#[derive(PartialEq)]
pub enum BFParseError {
    BraNoClose,
    BraNoOpen,
    CellMaxConstrOvrfl,
    CellMinConstrOvrfl,
    CellNoMore,
    CellNegativeIdx,
    IoStdinErr,
    IoReadingErr,
    IoUndefChar,
}

impl fmt::Debug for BFParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            BFParseError::BraNoClose => "Opening bracket without closing",
            BFParseError::BraNoOpen => "Closing bracket without opening",
            BFParseError::CellMaxConstrOvrfl => "Max cell value border exceeded",
            BFParseError::CellMinConstrOvrfl => "Min cell value border exceeded",
            BFParseError::CellNoMore => "Out of cells",
            BFParseError::CellNegativeIdx => "Cell pointer is negative",
            BFParseError::IoStdinErr => "Error while reading from stdin",
            BFParseError::IoReadingErr => "Error reading from file",
            BFParseError::IoUndefChar => "Undefined symbol",
        };
        write!(f, "{}", msg)
    }
}

const ERR_PROMPT: &str = ">>> ";
// a crutch, vec of error types that allowed to print values
const ERR_PRINTABLE_VALS: [BFParseError; 1] = [
    BFParseError::IoUndefChar
];

type ErrArgs = Option<String>;
type ErrFilename = Option<String>;
type ErrPos = Option<usize>;

pub struct BFError {
    e_file: ErrFilename,
    e_type: BFParseError,
    e_pos: ErrPos,
    e_args: ErrArgs
}

impl BFError {
    pub fn new(err: BFParseError, pos: ErrPos, args: ErrArgs) -> Self {
        BFError {
            e_file: env::args().nth(1),
            e_type: err, 
            e_pos: pos, 
            e_args: args
        }
    }
}

impl fmt::Debug for BFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: Vec<String> = Vec::new();
        
        if self.e_file.is_some() {
            let field = format!("File: {}", self.e_file.as_ref().unwrap());
            result.push(field);
        }

        let field = format!("Description: {:?}", self.e_type);
        result.push(field);

        if self.e_args.is_some() && ERR_PRINTABLE_VALS.contains(&self.e_type) {
            let field = format!("Value: '{}'", self.e_args.as_ref().unwrap());
            result.push(field);
        }

        if self.e_pos.is_some() {
            const ERR_AREA_RANGE: usize = 10;
            let pos = self.e_pos.as_ref().unwrap();
            let err_area_left = max(*pos as isize - ERR_AREA_RANGE as isize, 0) as usize;
            let err_marker = (0..pos-err_area_left).map(|_| " ").collect::<String>() + "^";
            let prog_area = PROG.with(|prog| {
                let borrowed_prog = prog.borrow();
                let err_area_right = min(pos + ERR_AREA_RANGE, borrowed_prog.len()-1); //deal with whitespaces
                String::from(&borrowed_prog[err_area_left..=err_area_right])
            });
            let field = format!("Position: {}\n{area}\n{marker}", 
                pos, 
                area = prog_area, 
                marker = err_marker);
            result.push(field);
        }

        let res_msg = result.iter()
            .map(|s| String::from(ERR_PROMPT) + s)
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", res_msg)
    }
}
