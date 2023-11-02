use std::collections::HashMap;

use colored::{Colorize, Color};
use pg_sugar_tests::view::{
  TableViewOptions, 
  TableViewOptionsData, 
  TableViewOptionsGrid, 
  TableViewOptionsHeader, 
  TableViewOptionsSeparator, 
  TableViewRow, 
  TableViewColumn, TableViewAlignment, 
};

#[test]
fn len() {
  let (options, data) = prepare();
  
  assert_eq!(
    data.len(&TableViewColumn { name: "field".into(), title: vec![], data_aligment: TableViewAlignment::Left }, &options),
    Some(8)
  );
  assert_eq!(
    data.len(&TableViewColumn { name: "any".into(), title: vec![], data_aligment: TableViewAlignment::Left }, &options),
    None
  );
}

#[test]
fn to_string() {
  let (options, data) = prepare();

  assert_eq!(
    data.to_colored_string(
      &vec![
        (&TableViewColumn { name: "field".into(), title: vec![], data_aligment: TableViewAlignment::Right }, 8),
        (&TableViewColumn { name: "f2".into(), title: vec![], data_aligment: TableViewAlignment::Left }, 5),
        (&TableViewColumn { name: "f3".into(), title: vec![], data_aligment: TableViewAlignment::Left }, 6),
      ], 
      &options
    ).to_string(),
    vec![
      "║".color(options.grid.color).to_string(),
      " ".into(), "123456".color(Color::Red).to_string(), " ".into(),
      "║".color(options.grid.color).to_string(),
      " ".into(), "123".color(Color::Red).to_string(), " ".into(),
      "║".color(options.grid.color).to_string(),
      " ".into(), "abcd".color(Color::Red).to_string(), " ".into(),
      "║".color(options.grid.color).to_string(),
      "\n".into(),
      "║".color(options.grid.color).to_string(),
      "    ".into(), "789".color(Color::Red).to_string(), " ".into(),
      "║".color(options.grid.color).to_string(),
      " ".into(), "456".color(Color::Blue).to_string(), " ".into(),
      "║".color(options.grid.color).to_string(),
      "      ".into(),
      "║".color(options.grid.color).to_string(),
      "\n".into(),
      "║".color(options.grid.color).to_string(),
      "        ".into(),
      "║".color(options.grid.color).to_string(),
      " ".into(), "789".color(Color::Red).to_string(), " ".into(),
      "║".color(options.grid.color).to_string(),
      "      ".into(),
      "║".color(options.grid.color).to_string(),
    ].join("")
  );
}

fn prepare() -> (TableViewOptions, TableViewRow) {
  (
    TableViewOptions { 
      data: TableViewOptionsData {
        prefix: " ".into(),
        postfix: " ".into(),
      }, 
      grid: TableViewOptionsGrid::new(), 
      separtor: TableViewOptionsSeparator::new(),
      header: TableViewOptionsHeader::Simple, 
    }, 
    TableViewRow::Data(HashMap::from([
      (
        "field".into(), 
        vec![ "123456".color(Color::Red), "789".color(Color::Red)  ]
      ),
      (
        "f2".into(), 
        vec![ "123".color(Color::Red), "456".color(Color::Blue), "789".color(Color::Red)  ]
      ),
      (
        "f3".into(), 
        vec![ "abcd".color(Color::Red),  ]
      )
    ]))
  )
}