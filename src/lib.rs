#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl std::default::Default for Color {
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Attribute {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}
impl std::default::Default for Attribute {
    fn default() -> Self {
        Attribute::Regular
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Align {
    Center,
    CenterPadded { padl: usize, padr: usize },
    Left,
    LeftPadded { padl: usize },
    Right,
    RightPadded { padr: usize },
}
impl std::default::Default for Align {
    fn default() -> Self {
        Align::Center
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Style {
    fg: Color,
    bg: Color,
    attribute: Attribute,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cell {
    value: String,
    align: Align,
    style: Style,
}
impl<T: std::fmt::Display> From<T> for Cell {
    fn from(val: T) -> Self {
        Cell {
            value: val.to_string(),
            align: Align::default(),
            style: Style::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Border {
    Single,
    Bold,
    Double,
    // TODO
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Row {
    HorizontalBorder(Border),
    FlexibleHeight(Vec<Cell>),
    FixedHeight { height: usize, cells: Vec<Cell> },
}
impl Row {
    fn border(&self) -> Option<Border> {
        match self {
            Row::HorizontalBorder(b) => Some(*b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Column {
    VerticalBorder(Border),
    FlexibleWidth,
    FixedWidth(usize),
}
impl Column {
    fn border(&self) -> Option<Border> {
        match self {
            Column::VerticalBorder(b) => Some(*b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Table {
    cols: Vec<Column>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            cols: columns,
            rows: Vec::new(),
        }
    }
    pub fn append_row(&mut self, row: Row) {
        self.rows.push(row);
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 1. calculate each column width.
        let widths: Vec<usize> = self
            .cols
            .iter()
            .map(|col| match col {
                Column::VerticalBorder(_) => 1,
                Column::FlexibleWidth => {
                    // maximum width of all rows
                    todo!();
                }
                Column::FixedWidth(w) => *w,
            })
            .collect();

        // 2. calculate each row height.
        let _heights: Vec<usize> = self
            .rows
            .iter()
            .map(|row| match row {
                Row::HorizontalBorder(_) => 1,
                Row::FlexibleHeight(_) => {
                    // maximum width of all columns
                    todo!()
                }
                Row::FixedHeight { height, .. } => *height,
            })
            .collect();

        // 3. render the table from top-left to bottom-right.
        for (ri, row) in self.rows.iter().enumerate() {
            let mut buf = Vec::new();
            match row {
                Row::HorizontalBorder(_) => {
                    for (ci, col) in self.cols.iter().enumerate() {
                        let c = get_border_char(
                            ri,
                            self.rows.len(),
                            ci,
                            self.cols.len(),
                            row.border(),
                            col.border(),
                        );
                        buf.extend_from_slice(&fill(c, widths[ci]));
                    }
                }
                Row::FixedHeight { cells, .. } | Row::FlexibleHeight(cells) => {
                    let mut cell_idx = 0;
                    for (ci, col) in self.cols.iter().enumerate() {
                        match col {
                            Column::VerticalBorder(_) => {
                                let c = get_border_char(
                                    ri,
                                    self.rows.len(),
                                    ci,
                                    self.cols.len(),
                                    row.border(),
                                    col.border(),
                                );
                                buf.extend_from_slice(&fill(c, widths[ci]));
                            }
                            _ => {
                                let cell: Vec<_> = cells[cell_idx].value.chars().collect();
                                buf.extend_from_slice(&cell);
                                cell_idx += 1;
                            }
                        }
                    }
                }
            }
            let line: String = buf.iter().collect();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

fn fill(c: char, width: usize) -> Vec<char> {
    vec![c; width]
}

fn get_border_char(
    row_idx: usize,
    rows: usize,
    col_idx: usize,
    cols: usize,
    horizontal: Option<Border>,
    vertical: Option<Border>,
) -> char {
    if horizontal.is_none() {
        '│'
    } else if vertical.is_none() {
        '─'
    } else {
        let mut bitmap = 0;
        if row_idx > 0 {
            bitmap |= 0b0001;
        }
        if row_idx + 1 < rows {
            bitmap |= 0b0100;
        }
        if col_idx > 0 {
            bitmap |= 0b1000;
        }
        if col_idx + 1 < cols {
            bitmap |= 0b0010;
        }
        match bitmap {
            0b0110 => '┌',
            0b1110 => '┬',
            0b1100 => '┐',
            0b0111 => '├',
            0b1111 => '┼',
            0b1101 => '┤',
            0b0011 => '└',
            0b1011 => '┴',
            0b1001 => '┘',
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cells_1() {
        let mut table = Table::new(vec![
            Column::VerticalBorder(Border::Single),
            Column::VerticalBorder(Border::Single),
        ]);
        table.append_row(Row::HorizontalBorder(Border::Single));
        table.append_row(Row::HorizontalBorder(Border::Single));
        let expected = r#"┌┐
└┘
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);
    }

    #[test]
    fn test_no_cells_2() {
        let mut table = Table::new(vec![
            Column::VerticalBorder(Border::Single),
            Column::FixedWidth(3),
            Column::VerticalBorder(Border::Single),
            Column::FixedWidth(3),
            Column::VerticalBorder(Border::Single),
        ]);
        table.append_row(Row::HorizontalBorder(Border::Single));
        table.append_row(Row::HorizontalBorder(Border::Single));
        table.append_row(Row::HorizontalBorder(Border::Single));
        let expected = r#"┌───┬───┐
├───┼───┤
└───┴───┘
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);
    }

    #[test]
    fn test_fixed_size_cell() {
        let mut table = Table::new(vec![
            Column::VerticalBorder(Border::Single),
            Column::FixedWidth(3),
            Column::VerticalBorder(Border::Single),
            Column::FixedWidth(3),
            Column::VerticalBorder(Border::Single),
        ]);
        table.append_row(Row::HorizontalBorder(Border::Single));
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec!["abc".into(), "123".into()],
        });
        table.append_row(Row::HorizontalBorder(Border::Single));
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec!["def".into(), "456".into()],
        });
        table.append_row(Row::HorizontalBorder(Border::Single));
        let expected = r#"┌───┬───┐
│abc│123│
├───┼───┤
│def│456│
└───┴───┘
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);
    }
}
