use std::fmt::{Display, Formatter, Result};



#[derive(PartialEq)]
pub enum Position {
    Cascade(usize),
    Foundations,
    Freecells
}


impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Position::Cascade(pos) => write!(f, "Cascade {}", pos),
            Position::Foundations => write!(f, "the Foundations"),
            Position::Freecells => write!(f, "the Freecells"),
        }
    }
}