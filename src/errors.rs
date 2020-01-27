use std::{
    fmt,
    env,
    cmp::{max, min}
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
type ErrArea = Option<String>;

pub struct BFError {
    e_file: ErrFilename,
    e_type: BFParseError,
    e_pos: ErrPos,
    e_area: ErrArea,
    e_args: ErrArgs
}

impl BFError {
    const ERR_AREA_RANGE: usize = 10;
    
    pub fn new(err: BFParseError, pos: ErrPos, args: ErrArgs) -> Self {
        BFError {
            e_file: env::args().nth(1),
            e_type: err, 
            e_pos: pos, 
            e_area: None,
            e_args: args
        }
    }
}

impl BFError {
    pub fn create_err_area(&mut self, prog: &Vec<char>) {
        if self.e_pos.is_some() {
            let pos = self.e_pos.as_ref().unwrap();
            let err_area_left = max(*pos as isize - Self::ERR_AREA_RANGE as isize, 0) as usize;
            let err_marker = (0..pos-err_area_left).map(|_| " ").collect::<String>() + "^";
            let err_area_right = min(pos + Self::ERR_AREA_RANGE, prog.len()-1); //deal with whitespaces
            let err_area = &prog[err_area_left..=err_area_right]
                .into_iter()
                .collect::<String>();
            self.e_area = Some(format!("{}\n{}", err_area, err_marker));
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
            let pos = self.e_pos.as_ref().unwrap();
            let area = match self.e_area.as_ref() {
                Some(area) => String::from("\n") + area,
                None => "".to_string()
            };
            let field = format!("Position: {}{}", pos, area);
            result.push(field);
        }

        let res_msg = result.iter()
            .map(|s| String::from(ERR_PROMPT) + s)
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", res_msg)
    }
}
