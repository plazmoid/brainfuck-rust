pub struct BFParseError;

impl<'a> BFParseError {
    pub const BRA_NO_CLOSE: &'a str = "Opening bracket without closing";
    pub const BRA_NO_OPEN: &'a str = "Closing bracket without opening";
    pub const CELL_MAX_CONSTR_OVRFL: &'a str = "Max cell value border exceeded";
    pub const CELL_MIN_CONSTR_OVRFL: &'a str = "Min cell value border exceeded";
    pub const CELL_NO_MORE: &'a str = "Out of cells";
    pub const CELL_NEGATIVE_IDX: &'a str = "Cell pointer is negative";
    pub const IO_STDIN_ERR: &'a str = "Error while reading from stdin";
    pub const IO_READING_ERR: &'a str = "Error reading from file";
    pub const IO_UNDEF_CHAR: &'a str = "Undefined symbol";
}

pub fn err_msg_pos(msg: String, pos: usize) -> String {
    format!("{} at pos {}", msg, pos)
}
