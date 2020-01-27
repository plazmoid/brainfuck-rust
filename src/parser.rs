use std::collections::HashMap;
use std::io::{stdin, Read};

use crate::errors::{
    BFError, 
    BFParseError,
    BFParseError::*
};

use crate::automata::Automata;

pub struct Reader {
    head_ptr: usize, // rw-head
    brackets: HashMap<usize, usize>, // double brackets pairs {pos1: pos2, pos2: pos1}
    program: Vec<char>, // program's symbols
    errors: Vec<BFError>,
    raw_program: String,
    automata: Automata
}

impl Reader {
    pub const MAX_PROG_LEN: usize = 1000000;

    pub fn new() -> Self {
        Reader {
            head_ptr: 0,
            brackets: HashMap::new(),
            program: Vec::new(),
            errors: Vec::new(),
            raw_program: String::new(),
            automata: Automata::new()
        }
    }

    // move all shit to iterator
    // fun that counts row/col from head_ptr and \ns
    // remove anime and Cursor
    fn next(&mut self) -> Option<&char> {
        let val = self.program.get(self.head_ptr);
        self.head_ptr += 1; 
        val
    }

    fn goto_bra_pair(&mut self, idx: usize) {
        self.head_ptr = self.brackets[&idx]
    }

    fn has_printable(&self) -> bool {
        self.automata.chars_to_print.len() > 0
    }

    fn get_output(&self) -> String {
        self.automata.chars_to_print.iter().collect::<String>()
    }

    fn analyse_brackets(&mut self) {
        let mut unclosed: Vec<usize> = Vec::new();
        for (i, ch) in self.program.iter().enumerate() {
            match ch {
                '[' => {
                    unclosed.push(i);
                }
                ']' => {
                    if let Some(pos) = unclosed.pop() {
                        self.brackets.insert(i, pos);
                        self.brackets.insert(pos, i);
                    } else {
                        self.errors.push(BFError::new(BraNoOpen, Some(i), None));
                    }
                }
                _ => (),
            }
        }

        unclosed.into_iter().for_each(|pos| {
            self.errors.push(BFError::new(BraNoClose, Some(pos), None))
        });
    }

    pub fn parse(&mut self, program: String) -> Result<(), &Vec<BFError>> {
        self.raw_program = program;
        self.program = self.raw_program.chars().collect();
        self.analyse_brackets();

        loop {
            let curr_symbol = match self.next() {
                Some(s) => *s,
                None => break,
            };

            let result: Result<(), BFParseError> = match curr_symbol {
                '+' => self.automata.inc(),
                '-' => self.automata.dec(),
                '>' => self.automata.rmove(),
                '<' => self.automata.lmove(),
                '.' => self.automata.out(),
                ',' => {
                    match stdin().bytes().next() {
                        Some(c) => match c {
                            Ok(chr) => self.automata.set(chr as u8),
                            Err(_) => Err(IoStdinErr),
                        },
                        None => Err(IoStdinErr),
                    }
                },
                '[' => {
                    if self.automata.get() == 0 {
                        self.goto_bra_pair(self.head_ptr - 1)
                    }
                    Ok(())
                },
                ']' => {
                    if self.automata.get() != 0 {
                        self.goto_bra_pair(self.head_ptr - 1)
                    }
                    Ok(())
                },
                ' ' | '\n' | '\t' => Ok(()),
                _ => Err(IoUndefChar),
            };
            if result.is_err() {
                let err = BFError::new(
                    result.unwrap_err(), 
                    Some(self.head_ptr), 
                    Some(curr_symbol.to_string())
                );
                self.errors.push(err)
            }
        };
        
        if self.errors.len() > 0 {
            let prog_lines = self.raw_program.split("\n");
            for err in self.errors.iter_mut() {
                err.create_err_area(&self.program);
            }
            /*self.errors.iter_mut().for_each(|err| {
                err.create_err_area(&self.program)
            }); FIXIT!!1 */
            return Err(&self.errors);
        }
        if self.has_printable() {
            print!("{}", self.get_output());
        }
        Ok(())
    }
}

/* TODO: rewrite this sheeet
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_right_layout() {
        let br = String::from("[[]]");
        let mut result: HashMap<usize, usize> = HashMap::new();
        result.insert(0, 3);
        result.insert(3, 0);
        result.insert(1, 2);
        result.insert(2, 1);
        assert_eq!(result, analyse_brackets(&br).unwrap());
    }

    #[test]
    #[should_panic]
    fn brackets_unclosed_left() {
        let br = String::from("[[][]");
        let result: HashMap<usize, usize> = HashMap::new();
        assert_eq!(result, analyse_brackets(&br).unwrap());
    }

    #[test]
    #[should_panic]
    fn brackets_unclosed_right() {
        let br = String::from("[][]]");
        let result: HashMap<usize, usize> = HashMap::new();
        assert_eq!(result, analyse_brackets(&br).unwrap());
    }

    #[test]
    fn parser_unknown_symbol() {
        let input = String::from("++++++q");
        let result = parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn parser_negative_cell_index() {
        let input = String::from("++++<");
        let result = parse(input);
        assert!(result.is_err());
    }
}
*/