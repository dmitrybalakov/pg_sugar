use std::collections::HashMap;

use colored::{Colorize, Color};
use pg_sugar_tests::view::{
  TableViewOptions, 
  TableViewRow, 
  TableView, 
};

use super::row::columns_vec;

#[test]
fn to_string() {
  let table = prepare();

  assert_eq!(
    table.to_string(),
    vec![
      "╔═══╦═══════════╦════════════╗".color(table.options.grid.color).to_string(),
      "\n".into(),
      "║".color(table.options.grid.color).to_string(),
      " ".into(), "*".color(Color::Red).to_string(), " ".into(),
      "║".color(table.options.grid.color).to_string(),
      "   ".into(), "name".color(Color::Red).to_string(), "    ".into(),
      "║".color(table.options.grid.color).to_string(),
      "   ".into(), "value".color(Color::Red).to_string(), "    ".into(),
      "║".color(table.options.grid.color).to_string(),
      "\n".into(),
      "║".color(table.options.grid.color).to_string(),
      "   ".into(),
      "║".color(table.options.grid.color).to_string(),
      " ".into(), "[VARCHAR]".color(Color::Green).to_string(), " ".into(),
      "║".color(table.options.grid.color).to_string(),
      "  ".into(), "[JSONB]".color(Color::Green).to_string(), "   ".into(),
      "║".color(table.options.grid.color).to_string(),
      "\n".into(),
      "╠═══╬═══════════╬════════════╣".color(table.options.grid.color).to_string(),
      "\n".into(),
      "║".color(table.options.grid.color).to_string(),
      " 1 ".into(),
      "║".color(table.options.grid.color).to_string(),
      " field     ".into(),
      "║".color(table.options.grid.color).to_string(),
      " ".into(), "'123'".into(), "      ".into(),
      "║".color(table.options.grid.color).to_string(),
      "\n".into(),
      "║".color(table.options.grid.color).to_string(),
      " 2 ".into(),
      "║".color(table.options.grid.color).to_string(),
      " json_v    ".into(),
      "║".color(table.options.grid.color).to_string(),
      " ".into(), "{".into(), "          ".into(),
      "║".color(table.options.grid.color).to_string(),
      "\n".into(),
      "║".color(table.options.grid.color).to_string(),
      "   ".into(),
      "║".color(table.options.grid.color).to_string(),
      "           ".into(),
      "║".color(table.options.grid.color).to_string(),
      " ".into(), r#"  "a": 123"#.color(Color::Blue).to_string(), " ".into(),
      "║".color(table.options.grid.color).to_string(),
      "\n".into(),
      "║".color(table.options.grid.color).to_string(),
      "   ".into(),
      "║".color(table.options.grid.color).to_string(),
      "           ".into(),
      "║".color(table.options.grid.color).to_string(),
      " ".into(), r#"}"#.into(), "          ".into(),
      "║".color(table.options.grid.color).to_string(),
      "\n".into(),
      "╚═══╩═══════════╩════════════╝".color(table.options.grid.color).to_string(),
    ].join("")
  );
}

fn prepare() -> TableView {
  TableView {
    options: TableViewOptions::new(),
    columns: columns_vec().into_iter().map(|x| x.0).collect(),
    rows: vec![
      TableViewRow::Header,
      TableViewRow::Data(
        HashMap::from([
          ("".into(), vec![ "1".into() ]),
          ("name".into(), vec![ "field".into() ]),
          ("value".into(), vec![ "'123'".into() ]),
        ]),
      ),
      TableViewRow::Data(
        HashMap::from([
          ("".into(), vec![ "2".into() ]),
          ("name".into(), vec![ "json_v".into() ]),
          ("value".into(), vec![ "{".into(), r#"  "a": 123"#.color(Color::Blue), "}".into() ]),
        ]),
      ),
      TableViewRow::Bottom,
    ]
  }
}