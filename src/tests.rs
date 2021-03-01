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
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
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
        Column::Cells {
            width: CellSize::Fixed(3),
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(3),
        },
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

    let mut table = Table::new(vec![
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(3),
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(3),
        },
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(3),
        cells: vec![],
    });
    table.append_row(Border::Single.into());
    let expected = r#"┌───┬───┐
│   │   │
│   │   │
│   │   │
└───┴───┘
"#
    .to_owned();
    assert_eq!(table.to_string(), expected);
}

#[test]
fn test_fixed_size_cell() {
    let mut table = Table::new(vec![
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(3),
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(3),
        },
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
        cells: vec!["abc".into(), "123".into()],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
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
        Column::Cells {
            width: CellSize::Fixed(5),
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(5),
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(5),
        },
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
        cells: vec!["(1,1)".into()],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
        cells: vec!["(2,1)".into(), "(2,2)".into()],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
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
        Column::Cells {
            width: CellSize::Fixed(5),
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(1),
        },
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(2),
        cells: vec!["abcdefgh".into()],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(3),
        cells: vec!["abc".into(), "xy".into()],
    });
    table.append_row(Border::Single.into());
    let expected = r#"┌─────┬─┐
│abcde│ │
│ fgh │ │
├─────┼─┤
│ abc │x│
│     │y│
│     │ │
└─────┴─┘
"#
    .to_owned();
    assert_eq!(table.to_string(), expected);
}

#[test]
fn test_cjk() {
    let mut table = Table::new(vec![
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(4),
        },
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(2),
        },
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(3),
        cells: vec!["あい".into(), "うえお".into()],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(2),
        cells: vec!["abc".into()],
    });
    table.append_row(Border::Single.into());
    let expected = r#"┌────┬──┐
│あい│う│
│    │え│
│    │お│
├────┼──┤
│abc │  │
│    │  │
└────┴──┘
"#
    .to_owned();
    assert_eq!(table.to_string(), expected);
}

#[test]
fn test_combine_border() {
    let mut table = Table::new(vec![
        Border::Single.into(),
        Column::Cells {
            width: CellSize::Fixed(3),
        },
        Border::Double.into(),
        Column::Cells {
            width: CellSize::Fixed(3),
        },
        Border::Single.into(),
    ]);
    table.append_row(Border::Double.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
        cells: vec!["abc".into(), "123".into()],
    });
    table.append_row(Border::Single.into());
    table.append_row(Row::Cells {
        height: CellSize::Fixed(1),
        cells: vec!["def".into(), "456".into()],
    });
    table.append_row(Border::Double.into());
    let expected = r#"╒═══╦═══╕
│abc║123│
├───╫───┤
│def║456│
╘═══╩═══╛
"#
    .to_owned();
    assert_eq!(table.to_string(), expected);
}
