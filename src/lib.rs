#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Border {
    Single,
    Bold,
    Double,
    // TODO
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Attribute {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Align {
    Left { left_pad: usize },
    Center { left_pad: usize, right_pad: usize },
    Right { right_pad: usize },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Style {
    fg: Color,
    bg: Color,
    attribute: Attribute,
    align: Align,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cell {
    raw: String,
    style: Style,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Row {
    HorizontalBorder(Border),
    Cells(Cell),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Column {
    VerticalBorder(Border),
    FixedWidth { width: usize },
    FlexibleWidth,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Table {
    width: usize,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(width: usize, columns: Vec<Column>) -> Self {
        Self {
            width,
            columns,
            rows: Vec::new(),
        }
    }
    pub fn push_row(&mut self, row: Row) {
        self.rows.push(row);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trivial() {
        assert_eq!(2 + 2, 4);
    }
}
