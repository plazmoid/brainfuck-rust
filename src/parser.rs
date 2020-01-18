use std::io::{Read, stdin};
use std::collections::HashMap;

use crate::errors::{BFParseError, err_msg_pos};

fn analyse_brackets(buf: &String) -> Result<HashMap<usize, usize>, String> {
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
                    return Err(
                        err_msg_pos(BFParseError::BRA_NO_OPEN.to_string(), i))
                }
            }
            _ => ()
        }
    }

    match unclosed.pop() {
        Some(pos) => Err(err_msg_pos(BFParseError::BRA_NO_CLOSE.to_string(), pos)),
        None => Ok(brackets)
    }
}

pub fn parse(buf: String) -> Result<(), String> {
    const CELLS_AMOUNT: usize = 30;
    let mut cells: Vec<u8> = vec![0; CELLS_AMOUNT];
    let mut cell_ptr: usize = 0;
    let mut head_ptr = 0;
    let prog_symbols: Vec<char> = buf.chars().collect();
    let brackets = analyse_brackets(&buf)?;
    //eprintln!("buf: {:?}", prog_symbols);

    let res: Result<(), String> = loop {
        let s: char = match prog_symbols.get(head_ptr) {
            Some(s) => *s,
            None => break Ok(())
        };

        match s {
            '+' => {
                if cells[cell_ptr] == 255 {
                    break Err(BFParseError::CELL_MAX_CONSTR_OVRFL.to_string())
                }
                cells[cell_ptr] += 1;
            },
            '-' => {
                if cells[cell_ptr] == 0 {
                    break Err(BFParseError::CELL_MIN_CONSTR_OVRFL.to_string())
                }
                cells[cell_ptr] -= 1;
            },
            '>' => {
                cell_ptr += 1;
                if cell_ptr == CELLS_AMOUNT {
                    break Err(BFParseError::CELL_NO_MORE.to_string())
                }
            },
            '<' => {
                if cell_ptr == 0 {
                    break Err(BFParseError::CELL_NEGATIVE_IDX.to_string())
                }
                cell_ptr -= 1;
            },
            '.' => {
                print!("{}", cells[cell_ptr] as char);
            },
            ',' => {
                let ch: u8 = match stdin().bytes().next() {
                    Some(c) => match c {
                        Ok(chr) => chr as u8,
                        Err(_) => break Err(BFParseError::IO_STDIN_ERR.to_string())
                    },
                    None => break Err(BFParseError::IO_STDIN_ERR.to_string())
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
            ' ' | '\n' | '\t' => (),

            _ => {
                break Err(format!("{} '{}'", BFParseError::IO_UNDEF_CHAR, s))
            }
        }
        head_ptr += 1;
    };
    if let Err(e) = res {
        return Err(err_msg_pos(e, head_ptr));
    }
    //eprintln!("{:?}", cells);

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