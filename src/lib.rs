pub use ansi_term;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Align {
    Center,
    CenterPadded { padl: usize, padr: usize },
    Left,
    LeftPadded { padl: usize },
    Right,
    RightPadded { padr: usize },
}
impl Align {
    pub fn padded_amount(&self) -> usize {
        match self {
            Self::Center => 0,
            Self::Left => 0,
            Self::Right => 0,
            Self::CenterPadded { padl, padr } => padl + padr,
            Self::LeftPadded { padl } => *padl,
            Self::RightPadded { padr } => *padr,
        }
    }
}
impl std::default::Default for Align {
    fn default() -> Self {
        Align::Center
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub value: String,
    pub align: Align,
    pub style: ansi_term::Style,
}
impl<T: std::fmt::Display> From<T> for Cell {
    fn from(val: T) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::default(),
            style: ansi_term::Style::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Border {
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CellSize {
    Flexible,
    Fixed(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Row {
    HorizontalBorder(Border),
    Cells { height: CellSize, cells: Vec<Cell> },
}
impl Row {
    fn border(&self) -> Option<Border> {
        match self {
            Row::HorizontalBorder(b) => Some(*b),
            _ => None,
        }
    }
    fn cells(&self) -> Option<&Vec<Cell>> {
        match self {
            Row::Cells { cells, .. } => Some(cells),
            _ => None,
        }
    }
}
impl From<Border> for Row {
    fn from(b: Border) -> Self {
        Row::HorizontalBorder(b)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Column {
    VerticalBorder(Border),
    Cells { width: CellSize },
}
impl Column {
    fn border(&self) -> Option<Border> {
        match self {
            Column::VerticalBorder(b) => Some(*b),
            _ => None,
        }
    }
}
impl From<Border> for Column {
    fn from(b: Border) -> Self {
        Column::VerticalBorder(b)
    }
}

mod table;
pub use table::*;

#[cfg(test)]
mod tests;
