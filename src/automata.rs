use crate::errors::{
    BFParseError,
    BFParseError::*
};

pub struct Automata {
    cells: Vec<u8>,
    cell_ptr: usize,
    pub chars_to_print: Vec<char>
}

impl Automata {
    const CELLS_AMOUNT: usize = 30000;

    pub fn new() -> Self {
        Automata {
            cells: vec![0; Automata::CELLS_AMOUNT],
            cell_ptr: 0,
            chars_to_print: Vec::new()
        }
    }

    pub fn inc(&mut self) -> Result<(), BFParseError> {
        if self.cells[self.cell_ptr] == 255 {
            Err(CellMaxConstrOvrfl)
        } else {
            self.cells[self.cell_ptr] += 1;
            Ok(())
        }
    }

    pub fn dec(&mut self) -> Result<(), BFParseError> {
        if self.cells[self.cell_ptr] == 0 {
            Err(CellMinConstrOvrfl)
        } else {
            self.cells[self.cell_ptr] -= 1;
            Ok(())
        }
    }

    pub fn lmove(&mut self) -> Result<(), BFParseError> {
        if self.cell_ptr == 0 {
            Err(CellNegativeIdx)
        } else {
            self.cell_ptr -= 1;
            Ok(())
        }
    }

    pub fn rmove(&mut self) -> Result<(), BFParseError> {
        if self.cell_ptr == Automata::CELLS_AMOUNT {
            Err(CellNoMore)
        } else {
            self.cell_ptr += 1;
            Ok(())
        }
    }

    pub fn set(&mut self, c: u8) -> Result<(), BFParseError> {
        self.cells[self.cell_ptr] = c;
        Ok(())
    }

    pub fn out(&mut self) -> Result<(), BFParseError> {
        self.chars_to_print.push(self.cells[self.cell_ptr] as char);
        Ok(())
    }

    pub fn get(&self) -> u8 {
        self.cells[self.cell_ptr]
    }
}