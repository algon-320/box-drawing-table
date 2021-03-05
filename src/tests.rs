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
    table.append_row(Row::fixed_height(1, vec![]));
    let expected = r#"││
"#
    .to_owned();
    assert_eq!(table.to_string(), expected);
}

#[test]
fn test_empty_cell() {
    let mut table = Table::new(vec![
        Border::Single.into(),
        Column::fixed_width(3),
        Border::Single.into(),
        Column::fixed_width(3),
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
        Column::fixed_width(3),
        Border::Single.into(),
        Column::fixed_width(3),
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(3, Vec::new()));
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
        Column::fixed_width(3),
        Border::Single.into(),
        Column::fixed_width(3),
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(
        1,
        vec![Cell::left("abc"), Cell::left("123")],
    ));
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(
        1,
        vec![Cell::left("def"), Cell::left("456")],
    ));
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
        Column::fixed_width(5),
        Border::Single.into(),
        Column::fixed_width(5),
        Border::Single.into(),
        Column::fixed_width(5),
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(1, vec![Cell::left("(1,1)")]));
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(
        1,
        vec![Cell::left("(2,1)"), Cell::left("(2,2)")],
    ));
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(
        1,
        vec![
            Cell::left("(3,1)"),
            Cell::left("(3,2)"),
            Cell::left("(3,3)"),
        ],
    ));
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
        Column::fixed_width(5),
        Border::Single.into(),
        Column::fixed_width(1),
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(2, vec![Cell::left("abcdefgh")]));
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(
        3,
        vec![Cell::left("abc"), Cell::left("xy")],
    ));
    table.append_row(Border::Single.into());
    let expected = r#"┌─────┬─┐
│abcde│ │
│fgh  │ │
├─────┼─┤
│abc  │x│
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
        Column::fixed_width(4),
        Border::Single.into(),
        Column::fixed_width(2),
        Border::Single.into(),
    ]);
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(
        3,
        vec![Cell::left("あい"), Cell::left("うえお")],
    ));
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(2, vec![Cell::left("abc")]));
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
        Column::fixed_width(3),
        Border::Double.into(),
        Column::fixed_width(3),
        Border::Single.into(),
    ]);
    table.append_row(Border::Double.into());
    table.append_row(Row::fixed_height(
        1,
        vec![Cell::left("abc"), Cell::left("123")],
    ));
    table.append_row(Border::Single.into());
    table.append_row(Row::fixed_height(
        1,
        vec![Cell::left("def"), Cell::left("456")],
    ));
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
