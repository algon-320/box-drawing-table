use super::*;

#[derive(Debug, Clone, PartialEq)]
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
        match (horizontal, vertical) {
            (None, None) => panic!(""),
            (None, Some(vertical)) => match vertical {
                Border::Single => "│",
                Border::Double => "║",
            },
            (Some(horizontal), None) => match horizontal {
                Border::Single => "─",
                Border::Double => "═",
            },
            (Some(horizontal), Some(vertical)) => {
                // this represents the existence of adjacent borders
                let mut adjacent_border = 0;
                if row_idx > 0 {
                    adjacent_border |= 0b0001; // up
                }
                if row_idx + 1 < self.rows.len() {
                    adjacent_border |= 0b0100; // right
                }
                if col_idx > 0 {
                    adjacent_border |= 0b1000; // down
                }
                if col_idx + 1 < self.cols.len() {
                    adjacent_border |= 0b0010; // left
                }

                use Border::*;
                match adjacent_border {
                    0b0110 => match (horizontal, vertical) {
                        (Single, Single) => "┌",
                        (Double, Double) => "╔",
                        (Single, Double) => "╓",
                        (Double, Single) => "╒",
                    },
                    0b1110 => match (horizontal, vertical) {
                        (Single, Single) => "┬",
                        (Double, Double) => "╦",
                        (Single, Double) => "╥",
                        (Double, Single) => "╤",
                    },
                    0b1100 => match (horizontal, vertical) {
                        (Single, Single) => "┐",
                        (Double, Double) => "╗",
                        (Single, Double) => "╖",
                        (Double, Single) => "╕",
                    },
                    0b0111 => match (horizontal, vertical) {
                        (Single, Single) => "├",
                        (Double, Double) => "╠",
                        (Single, Double) => "╟",
                        (Double, Single) => "╞",
                    },
                    0b1111 => match (horizontal, vertical) {
                        (Single, Single) => "┼",
                        (Double, Double) => "╬",
                        (Single, Double) => "╫",
                        (Double, Single) => "╪",
                    },
                    0b1101 => match (horizontal, vertical) {
                        (Single, Single) => "┤",
                        (Double, Double) => "╣",
                        (Single, Double) => "╢",
                        (Double, Single) => "╡",
                    },
                    0b0011 => match (horizontal, vertical) {
                        (Single, Single) => "└",
                        (Double, Double) => "╚",
                        (Single, Double) => "╙",
                        (Double, Single) => "╘",
                    },
                    0b1011 => match (horizontal, vertical) {
                        (Single, Single) => "┴",
                        (Double, Double) => "╩",
                        (Single, Double) => "╨",
                        (Double, Single) => "╧",
                    },
                    0b1001 => match (horizontal, vertical) {
                        (Single, Single) => "┘",
                        (Double, Double) => "╝",
                        (Single, Double) => "╜",
                        (Double, Single) => "╛",
                    },
                    0b0010 | 0b1000 => match (horizontal, vertical) {
                        (Single, _) => "─",
                        (Double, _) => "═",
                    },
                    0b0001 | 0b0100 => match (horizontal, vertical) {
                        (_, Single) => "│",
                        (_, Double) => "║",
                    },
                    _ => unimplemented!(),
                }
            }
        }
    }
}

fn fill(s: &str, width: usize) -> String {
    use unicode_width::UnicodeWidthStr;
    let s_width = s.width();
    assert!(width % s_width == 0);
    let times = width / s_width;
    s.repeat(times)
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use unicode_width::UnicodeWidthStr;

        // 1. calculate each column width.
        let widths: Vec<usize> = {
            let mut ws = Vec::new();
            let mut cell_idx = 0;
            for col in self.cols.iter() {
                match col {
                    Column::VerticalBorder(_) => ws.push(1),
                    Column::Cells { width } => {
                        match width {
                            CellSize::Flexible => {
                                // maximum width of all rows
                                let max_width = self
                                    .rows
                                    .iter()
                                    .filter_map(|row| {
                                        row.cells().and_then(|cells| cells.get(cell_idx))
                                    })
                                    .map(|cell| cell.value.width() + cell.align.padded_amount())
                                    .fold(1 /* defualt width*/, std::cmp::max);
                                ws.push(max_width);
                            }
                            CellSize::Fixed(w) => ws.push(*w),
                        }
                        cell_idx += 1;
                    }
                }
            }
            ws
        };

        // 2. render each row
        for (ri, row) in self.rows.iter().enumerate() {
            match row {
                Row::HorizontalBorder(_) => {
                    for (ci, col) in self.cols.iter().enumerate() {
                        let c = self.get_border(ri, ci, row.border(), col.border());
                        write!(f, "{}", fill(c, widths[ci]))?;
                    }
                    writeln!(f)?;
                }
                Row::Cells { height, cells } => {
                    use std::borrow::Cow;
                    enum ActualContent<'a> {
                        Border(String),
                        Text(Vec<Cow<'a, str>>),
                    }
                    struct ActualCell<'a> {
                        width: usize,
                        align: Align,
                        style: ansi_term::Style,
                        content: ActualContent<'a>,
                    }

                    let cells: Vec<ActualCell> = {
                        let mut buf = Vec::new();
                        let mut cells_iter = cells.iter();
                        for (ci, col) in self.cols.iter().enumerate() {
                            match col {
                                Column::Cells { .. } => match cells_iter.next() {
                                    Some(cell) => {
                                        let width = widths[ci];
                                        let wrap_opts = textwrap::Options::with_splitter(
                                            width - cell.align.padded_amount(),
                                            textwrap::NoHyphenation,
                                        );
                                        buf.push(ActualCell {
                                            width: width + cell.align.padded_amount(),
                                            align: cell.align,
                                            style: cell.style,
                                            content: ActualContent::Text(textwrap::wrap(
                                                cell.value.as_str(),
                                                wrap_opts,
                                            )),
                                        });
                                    }
                                    None => {
                                        // empty cell
                                        buf.push(ActualCell {
                                            width: widths[ci],
                                            align: Align::default(),
                                            style: ansi_term::Style::default(),
                                            content: ActualContent::Text(Vec::new()),
                                        });
                                    }
                                },
                                Column::VerticalBorder(b) => {
                                    let border_str = self.get_border(ri, ci, None, Some(*b));
                                    let repeated = fill(border_str, widths[ci]);
                                    let align = Align::default();
                                    buf.push(ActualCell {
                                        width: widths[ci] + align.padded_amount(),
                                        align: Align::default(),
                                        style: ansi_term::Style::default(),
                                        content: ActualContent::Border(repeated),
                                    });
                                }
                            }
                        }
                        buf
                    };

                    let height = match height {
                        CellSize::Flexible => {
                            // maximum width of all columns
                            cells
                                .iter()
                                .filter_map(|actual| match &actual.content {
                                    ActualContent::Text(text) => Some(text.len()),
                                    _ => None,
                                })
                                .fold(1, std::cmp::max)
                        }
                        CellSize::Fixed(h) => *h,
                    };

                    let mut lines = vec![String::new(); height];
                    for actual in cells {
                        match actual.content {
                            ActualContent::Text(text) => {
                                let text =
                                    text.into_iter().chain(std::iter::repeat(Cow::Borrowed("")));
                                for (buf, w) in lines.iter_mut().zip(text) {
                                    let sz = w.as_ref().width();
                                    let pad = actual.width - actual.align.padded_amount() - sz;
                                    let (padl, padr) = match actual.align {
                                        Align::Center => (pad / 2, pad - pad / 2),
                                        Align::Left => (0, pad),
                                        Align::Right => (pad, 0),
                                        Align::CenterPadded { padl, padr } => {
                                            (padl + (pad - padl - padr), padr)
                                        }
                                        Align::LeftPadded { padl } => (padl, pad - padl),
                                        Align::RightPadded { padr } => (pad - padr, padr),
                                    };
                                    assert_eq!(pad, padl + padr);
                                    buf.push_str(&fill(" ", padl));
                                    buf.push_str(&actual.style.paint(w.as_ref()).to_string());
                                    buf.push_str(&fill(" ", padr));
                                }
                            }
                            ActualContent::Border(border) => {
                                for buf in lines.iter_mut() {
                                    buf.push_str(
                                        &actual
                                            .style
                                            .paint(fill(&border, actual.width))
                                            .to_string(),
                                    );
                                }
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
