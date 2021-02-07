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
impl From<Border> for Row {
    fn from(b: Border) -> Self {
        Row::HorizontalBorder(b)
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
impl From<Border> for Column {
    fn from(b: Border) -> Self {
        Column::VerticalBorder(b)
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

    fn get_border(
        &self,
        row_idx: usize,
        col_idx: usize,
        horizontal: Option<Border>,
        vertical: Option<Border>,
    ) -> &str {
        if horizontal.is_none() {
            "│"
        } else if vertical.is_none() {
            "─"
        } else {
            let mut bitmap = 0;
            if row_idx > 0 {
                bitmap |= 0b0001;
            }
            if row_idx + 1 < self.rows.len() {
                bitmap |= 0b0100;
            }
            if col_idx > 0 {
                bitmap |= 0b1000;
            }
            if col_idx + 1 < self.cols.len() {
                bitmap |= 0b0010;
            }
            match bitmap {
                0b0110 => "┌",
                0b1110 => "┬",
                0b1100 => "┐",
                0b0111 => "├",
                0b1111 => "┼",
                0b1101 => "┤",
                0b0011 => "└",
                0b1011 => "┴",
                0b1001 => "┘",
                0b0010 => "─",
                0b1000 => "─",
                0b0001 => "│",
                0b0100 => "│",
                _ => unreachable!(),
            }
        }
    }
}

fn fill(c: &str, width: usize) -> String {
    c.repeat(width)
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use unicode_width::UnicodeWidthStr;

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
        let heights: Vec<usize> = self
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
            match row {
                Row::HorizontalBorder(_) => {
                    for (ci, col) in self.cols.iter().enumerate() {
                        let c = self.get_border(ri, ci, row.border(), col.border());
                        write!(f, "{}", fill(c, widths[ci]))?;
                    }
                    writeln!(f)?;
                }
                Row::FixedHeight { cells, .. } | Row::FlexibleHeight(cells) => {
                    let height = heights[ri];
                    let mut lines = vec![String::new(); height];
                    let mut cell_idx = 0;
                    for (ci, col) in self.cols.iter().enumerate() {
                        match col {
                            Column::VerticalBorder(_) => {
                                let c = self.get_border(ri, ci, row.border(), col.border());
                                for line in lines.iter_mut() {
                                    line.push_str(&fill(c, widths[ci]));
                                }
                            }
                            _ => {
                                let opts = textwrap::Options::new(widths[ci])
                                    .splitter(textwrap::NoHyphenation);
                                let wrapped = textwrap::wrap(
                                    &cells
                                        .get(cell_idx)
                                        .map(|cell| cell.value.as_str())
                                        .unwrap_or(""),
                                    opts,
                                )
                                .into_iter()
                                .chain(std::iter::repeat(std::borrow::Cow::Borrowed("")));
                                for (line, w) in lines.iter_mut().zip(wrapped) {
                                    line.push_str(w.as_ref());
                                    let sz = w.as_ref().width();
                                    line.push_str(&fill(" ", widths[ci] - sz));
                                }
                                cell_idx += 1;
                            }
                        }
                    }
                    for line in lines {
                        writeln!(f, "{}", line)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cells() {
        let mut table = Table::new(vec![Border::Single.into(), Border::Single.into()]);
        table.append_row(Border::Single.into());
        table.append_row(Border::Single.into());
        let expected = r#"┌┐
└┘
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);

        let mut table = Table::new(vec![Border::Single.into(), Border::Single.into()]);
        table.append_row(Border::Single.into());
        let expected = r#"──
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);

        let mut table = Table::new(vec![Border::Single.into(), Border::Single.into()]);
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec![],
        });
        let expected = r#"││
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);
    }

    #[test]
    fn test_empty_cell() {
        let mut table = Table::new(vec![
            Border::Single.into(),
            Column::FixedWidth(3),
            Border::Single.into(),
            Column::FixedWidth(3),
            Border::Single.into(),
        ]);
        table.append_row(Border::Single.into());
        table.append_row(Border::Single.into());
        table.append_row(Border::Single.into());
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
            Border::Single.into(),
            Column::FixedWidth(3),
            Border::Single.into(),
            Column::FixedWidth(3),
            Border::Single.into(),
        ]);
        table.append_row(Border::Single.into());
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec!["abc".into(), "123".into()],
        });
        table.append_row(Border::Single.into());
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec!["def".into(), "456".into()],
        });
        table.append_row(Border::Single.into());
        let expected = r#"┌───┬───┐
│abc│123│
├───┼───┤
│def│456│
└───┴───┘
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);
    }

    #[test]
    fn test_lacking_cell() {
        let mut table = Table::new(vec![
            Border::Single.into(),
            Column::FixedWidth(5),
            Border::Single.into(),
            Column::FixedWidth(5),
            Border::Single.into(),
            Column::FixedWidth(5),
            Border::Single.into(),
        ]);
        table.append_row(Border::Single.into());
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec!["(1,1)".into()],
        });
        table.append_row(Border::Single.into());
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec!["(2,1)".into(), "(2,2)".into()],
        });
        table.append_row(Border::Single.into());
        table.append_row(Row::FixedHeight {
            height: 1,
            cells: vec!["(3,1)".into(), "(3,2)".into(), "(3,3)".into()],
        });
        table.append_row(Border::Single.into());
        let expected = r#"┌─────┬─────┬─────┐
│(1,1)│     │     │
├─────┼─────┼─────┤
│(2,1)│(2,2)│     │
├─────┼─────┼─────┤
│(3,1)│(3,2)│(3,3)│
└─────┴─────┴─────┘
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);
    }

    #[test]
    fn test_wrapping() {
        let mut table = Table::new(vec![
            Border::Single.into(),
            Column::FixedWidth(5),
            Border::Single.into(),
        ]);
        table.append_row(Border::Single.into());
        table.append_row(Row::FixedHeight {
            height: 2,
            cells: vec!["abcdefgh".into()],
        });
        table.append_row(Border::Single.into());
        table.append_row(Row::FixedHeight {
            height: 3,
            cells: vec!["abc".into()],
        });
        table.append_row(Border::Single.into());
        let expected = r#"┌─────┐
│abcde│
│fgh  │
├─────┤
│abc  │
│     │
│     │
└─────┘
"#
        .to_owned();
        assert_eq!(table.to_string(), expected);
    }
}
