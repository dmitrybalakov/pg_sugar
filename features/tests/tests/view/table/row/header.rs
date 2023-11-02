use colored::{Color, Colorize, ColoredString};
use pg_sugar_tests::view::{
  TableViewOptions, 
  TableViewOptionsData, 
  TableViewOptionsGrid, 
  TableViewOptionsHeader, 
  TableViewOptionsSeparator, 
  TableViewRow, 
  TableViewColumn, 
  TableViewOptionsHeaderColumns, 
  TableViewAlignment
};

use crate::view::table::row::columns_vec;

#[test]
fn simple_len() {
  let (options, header) = prepare_simple();
  
  assert_eq!(
    header.len(&TableViewColumn { name: "any".into(), title: vec![], data_aligment: TableViewAlignment::Left }, &options),
    None
  );
}

#[test]
fn simple_to_string() {
  let (options, header) = prepare_simple();
  let binding = columns_vec();
  let columns = binding
    .iter()
    .map(|(row, size)| (row, *size))
    .collect();
  
  assert_eq!(
    header.to_colored_string(&columns, &options).to_string(),
    "╔═════╦═════════════╦═══════════╗".color(options.grid.color).to_string(),
  );
}

#[test]
fn columns_len() {
  let (options, header, columns) = prepare_columns();

  assert_eq!(header.len(&columns[0].0, &options), Some(5));
  assert_eq!(header.len(&columns[1].0, &options), Some(13));
  assert_eq!(header.len(&columns[2].0, &options), Some(11));
}

#[test]
fn columns_to_string() {
  let (options, header, columns) = prepare_columns();

  assert_eq!(
    header.to_colored_string(
      &columns
        .iter()
        .map(|(row, size)| (row, *size))
        .collect(), 
      &options
    ).to_string(),
    vec![
      "╔═════╦═════════════╦═══════════╗".color(options.grid.color).to_string(),
      format!(
        "{d}{p}{index}{p}{d}{p}  {name}   {p}{d}{p} {value} {p}{d}",
        d = "║".color(options.grid.color),
        p = ColoredString::from("  "),
        index = "*".color(Color::Red),
        name = "name".color(Color::Red),
        value = "value".color(Color::Red),
      ),
      format!(
        "{d}{p}{index}{p}{d}{p}{name}{p}{d}{p}{value}{p}{d}",
        d = "║".color(options.grid.color),
        p = ColoredString::from("  "),
        index = " ",
        name = "[VARCHAR]".color(Color::Green),
        value = "[JSONB]".color(Color::Green),
      ),
      "╠═════╬═════════════╬═══════════╣".color(options.grid.color).to_string(),
    ].join("\n")
  );
}

fn prepare_simple() -> (TableViewOptions, TableViewRow) {
  (options(TableViewOptionsHeader::Simple), TableViewRow::Header)
}

fn prepare_columns() -> (TableViewOptions, TableViewRow, Vec<(TableViewColumn, usize)>) {
  let options_columns = TableViewOptionsHeaderColumns {
    color: Color::Red,
    prefix: "  ".into(),
    postfix: "  ".into(),
    alignment: TableViewAlignment::Center,
  };

  (
    options(TableViewOptionsHeader::Columns(options_columns)), 
    TableViewRow::Header,
    columns_vec()
  )
}

fn options(header: TableViewOptionsHeader) -> TableViewOptions {
  TableViewOptions { 
    data: TableViewOptionsData::new(), 
    grid: TableViewOptionsGrid::new(), 
    separtor: TableViewOptionsSeparator::new(),
    header, 
  }
}