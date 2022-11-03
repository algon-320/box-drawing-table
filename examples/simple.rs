use box_drawing_table::*;

fn main() {
    let mut table = Table::new(vec![
        Border::Double.into(),
        Column::Cells {
            width: CellSize::Flexible,
        },
        Border::Double.into(),
        Column::Cells {
            width: CellSize::Flexible,
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(10),
        },
        Border::Double.into(),
    ]);
    table.append_row(Border::Double.into());
    table.append_row(Row::Cells {
        height: CellSize::Flexible,
        cells: vec![Cell::left(""), Cell::left("w=*"), Cell::left("w=10")],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Flexible,
        cells: vec![
            Cell::left("h=*"),
            Cell {
                value: "123456789012345".into(),
                align: Align::Left,
                style: ansi_term::Style::new(),
            },
            Cell {
                value: "123456789012345".into(),
                align: Align::Left,
                style: ansi_term::Style::new(),
            },
        ],
    });
    table.append_row(Border::Double.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(2),
        cells: vec![
            Cell::left("h=2"),
            Cell {
                value: "Left".into(),
                align: Align::Left,
                style: ansi_term::Style::new()
                    .bold()
                    .fg(ansi_term::Color::RGB(245, 66, 170)),
            },
            Cell {
                value: "Right".into(),
                align: Align::Right,
                style: ansi_term::Style::new()
                    .underline()
                    .on(ansi_term::Color::RGB(66, 206, 245)),
            },
        ],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Flexible,
        cells: vec![
            Cell::left("h=*"),
            Cell {
                value: "{padl:2, padr:1}".into(),
                align: Align::CenterPadded { padl: 2, padr: 1 },
                style: ansi_term::Style::new().strikethrough(),
            },
            Cell {
                value: "{padr:1}あいうえお1234567890かきくけこ".into(),
                align: Align::RightPadded { padr: 1 },
                style: ansi_term::Style::new().fg(ansi_term::Color::RGB(221, 245, 66)),
            },
        ],
    });
    table.append_row(Border::Double.into());

    print!("{}", table);
}
