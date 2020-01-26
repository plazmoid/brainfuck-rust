use std::collections::HashMap;
use std::io::{stdin, Read};

use crate::errors::{
    BFError, 
    BFParseError,
    BFParseError::*
};

fn analyse_brackets(buf: &String) -> Result<HashMap<usize, usize>, BFError> {
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
                    return Err(BFError::new(BraNoOpen, i, None));
                }
            }
            _ => (),
        }
    }

    match unclosed.pop() { 
        Some(pos) => Err(BFError::new(BraNoClose, pos, None)),
        None => Ok(brackets),
    }
}

pub fn parse(buf: String) -> Result<(), BFError> {
    const CELLS_AMOUNT: usize = 30000;
    let mut cells: Vec<u8> = vec![0; CELLS_AMOUNT];
    let mut cell_ptr: usize = 0;
    let mut head_ptr = 0;
    let mut curr_symbol = '\0';
    let prog_symbols: Vec<char> = buf.chars().collect();
    let brackets = analyse_brackets(&buf)?;
    //eprintln!("buf: {:?}", prog_symbols);

    let res: Result<(), BFParseError> = loop {
        curr_symbol = match prog_symbols.get(head_ptr) {
            Some(s) => *s,
            None => break Ok(()),
        };

        match curr_symbol {
            '+' => {
                if cells[cell_ptr] == 255 {
                    break Err(CellMaxConstrOvrfl);
                }
                cells[cell_ptr] += 1;
            }
            '-' => {
                if cells[cell_ptr] == 0 {
                    break Err(CellMinConstrOvrfl);
                }
                cells[cell_ptr] -= 1;
            }
            '>' => {
                cell_ptr += 1;
                if cell_ptr == CELLS_AMOUNT {
                    break Err(CellNoMore);
                }
            }
            '<' => {
                if cell_ptr == 0 {
                    break Err(CellNegativeIdx);
                }
                cell_ptr -= 1;
            }
            '.' => {
                print!("{}", cells[cell_ptr] as char);
            }
            ',' => {
                let ch: u8 = match stdin().bytes().next() {
                    Some(c) => match c {
                        Ok(chr) => chr as u8,
                        Err(_) => break Err(IoStdinErr),
                    },
                    None => break Err(IoStdinErr),
                };
                cells[cell_ptr] = ch;
            }
            '[' => {
                if cells[cell_ptr] == 0 {
                    head_ptr = brackets[&head_ptr];
                }
            }
            ']' => {
                if cells[cell_ptr] != 0 {
                    head_ptr = brackets[&head_ptr]
                }
            }
            ' ' | '\n' | '\t' => (),

            _ => break Err(IoUndefChar),
        }
        head_ptr += 1;
    };
    
    if let Err(e_type) = res {
        return Err(BFError::new(e_type, head_ptr, Some(curr_symbol.to_string())));
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
