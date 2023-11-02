use colored::Colorize;
use pg_sugar_tests::view::{
  TableViewOptions, 
  TableViewOptionsData, 
  TableViewOptionsGrid, 
  TableViewOptionsHeader, 
  TableViewOptionsSeparator, 
  TableViewRow, 
  TableViewColumn, 
  TableViewAlignment, 
};

use crate::view::table::row::columns_vec;

#[test]
fn bottom_len() {
  let (options, separator) = prepare();

  assert_eq!(
    separator.len(&TableViewColumn { name: "any".into(), title: vec![], data_aligment: TableViewAlignment::Left }, &options),
    None
  );
}

#[test]
fn bottom_to_string() {
  let (options, separator) = prepare();
  let binding = columns_vec();
  let columns = binding
    .iter()
    .map(|(row, size)| (row, *size))
    .collect();

  assert_eq!(
    separator.to_colored_string(&columns, &options).to_string(),
    "╚═════╩═════════════╩═══════════╝".color(options.grid.color).to_string()
  );
}

fn prepare() -> (TableViewOptions, TableViewRow) {
  (
    TableViewOptions { 
      data: TableViewOptionsData::new(), 
      grid: TableViewOptionsGrid::new(), 
      separtor: TableViewOptionsSeparator::new(),
      header: TableViewOptionsHeader::Simple, 
    }, 
    TableViewRow::Bottom
  )
}