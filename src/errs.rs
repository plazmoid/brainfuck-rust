use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum BFParseError {
    BraNoClose,
    BraNoOpen,
    CellValOvrfl
}

impl Error for BFParseError {


    /*
    pub const BRA_NO_CLOSE: &'static str = "Opening bracket without closing";
    pub const BRA_NO_OPEN: &'static str = "Closing bracket without opening";
    pub const CELL_VAL_OVRFL: &'static str = "Closing bracket without opening";*/
}

pub fn err_msg_pos(msg: &'static str, pos: usize) -> String {
    format!("{} at pos {}", msg, pos)        
}