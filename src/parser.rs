use std::io::{Read, stdin};
use std::error::Error;
use std::collections::HashMap;

use crate::errs::{ParseError, err_msg_pos};

fn analyse_brackets(buf: &String) -> Result<HashMap<usize, usize>, Error> {
    let mut unclosed: Vec<usize> = Vec::new();
    let mut brackets: HashMap<usize, usize> = HashMap::new();
    for (i, ch) in buf.chars().enumerate() {
        match ch {
            '[' => {
                unclosed.push(i);
            }
            ']' => {
                if let Some(pos) = unclosed.pop() {
                    brackets.insert(i, pos);
                    brackets.insert(pos, i);
                } else {
                    return Err(Error::new(err_msg_pos(ParseError::BRA_NO_OPEN, i))
                }
            }
            _ => ()
        }
    }

    match unclosed.pop() {        
        Some(pos) => Err(err_msg_pos(ParseError::BRA_NO_CLOSE, pos)),
        None => Ok(brackets)
    }
}

pub fn parse(buf: String) -> Result<(), &'static str> {
    const CELLS_AMOUNT: usize = 30;
    let mut cells: Vec<u8> = vec![0; CELLS_AMOUNT];
    let mut cell_ptr: usize = 0;
    let mut head_ptr = 0;
    let mut prog_symbols = buf.chars();
    let brackets = analyse_brackets(&buf)?;

    let res: Result<(), String> = loop {
        let s: char = match prog_symbols.nth(head_ptr) {
            Some(s) => s,
            None => break Ok(())
        };

        match s {
            '+' => {
                if cells[cell_ptr] == 255 {
                    break Err(String::from("Attempt to add with overflow"))
                }
                cells[cell_ptr] += 1;
            }
            '-' => {
                if cells[cell_ptr] == 0 {
                    break Err(String::from("Attempt to set a negative cell value"))
                }
                cells[cell_ptr] -= 1;
            }
            '>' => {
                cell_ptr += 1;
                if cell_ptr == CELLS_AMOUNT {
                    break Err(String::from("Exceeded max cells amount"))
                }
            }
            '<' => {
                if cell_ptr == 0 {
                    break Err(String::from("Attempt to use a cell with negative index"))
                }
                cell_ptr -= 1;
            }
            '.' => {
                println!("{}", cells[cell_ptr] as char);
            }
            ',' => {
                let ch: u8 = match stdin().bytes().next() {
                    Some(c) => match c {
                        Ok(chr) => chr as u8,
                        Err(e) => break Err(String::from(e.description()))
                    },
                    None => break Err(String::from("Error while reading from stdin"))
                };
                cells[cell_ptr] = ch;
            },
            '[' => {
                if cells[cell_ptr] == 0 {
                    head_ptr = brackets[&head_ptr];
                }
            },
            ']' => {
                if cells[cell_ptr] != 0 {
                    head_ptr = brackets[&head_ptr]
                }
            },
            _ => {
                break Err(format!("Undefined symbol '{}'", s))
            }
        }
        head_ptr += 1;
    };
    
    if let Err(e) = res {
        return Err(gen_err_str(e, head_ptr));
    }

    Ok(())
}


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
        assert!(parse(input).is_err());
    }
}