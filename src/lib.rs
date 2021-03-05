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
    pub fn padding_size(&self) -> usize {
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
        Align::Left
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub value: String,
    pub align: Align,
    pub style: ansi_term::Style,
}

use ansi_term::Style;
impl Cell {
    pub fn left<T: std::fmt::Display>(val: T) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::Left,
            style: Style::default(),
        }
    }
    pub fn left_with_style<T: std::fmt::Display>(val: T, style: Style) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::Left,
            style,
        }
    }

    pub fn right<T: std::fmt::Display>(val: T) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::Right,
            style: Style::default(),
        }
    }
    pub fn right_with_style<T: std::fmt::Display>(val: T, style: Style) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::Right,
            style,
        }
    }

    pub fn center<T: std::fmt::Display>(val: T) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::Center,
            style: Style::default(),
        }
    }
    pub fn center_with_style<T: std::fmt::Display>(val: T, style: Style) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::Center,
            style,
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
    pub fn flexible_height(cells: Vec<Cell>) -> Self {
        Self::Cells {
            height: CellSize::Flexible,
            cells,
        }
    }
    pub fn fixed_height(height: usize, cells: Vec<Cell>) -> Self {
        Self::Cells {
            height: CellSize::Fixed(height),
            cells,
        }
    }

    pub fn border(&self) -> Option<Border> {
        match self {
            Row::HorizontalBorder(b) => Some(*b),
            _ => None,
        }
    }
    pub fn cells(&self) -> Option<&Vec<Cell>> {
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
    pub fn flexible_width() -> Self {
        Self::Cells {
            width: CellSize::Flexible,
        }
    }
    pub fn fixed_width(width: usize) -> Self {
        Self::Cells {
            width: CellSize::Fixed(width),
        }
    }

    pub fn border(&self) -> Option<Border> {
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
