mod bottom;
mod data;
mod header;
mod separator;

use colored::{Color, Colorize};

use pg_sugar_tests::view::{TableViewColumn, TableViewAlignment};

pub fn columns_vec() -> Vec<(TableViewColumn, usize)> {
  vec![
    (
      TableViewColumn {
        name: "".into(),
        title: vec![ "*".color(Color::Red) ],
        data_aligment: TableViewAlignment::Left
      },
      5
    ),
    (
      TableViewColumn {
        name: "name".into(),
        title: vec![ 
          "name".color(Color::Red),
          "[VARCHAR]".color(Color::Green)
        ],
        data_aligment: TableViewAlignment::Left
      },
      13
    ),
    (
      TableViewColumn {
        name: "value".into(),
        title: vec![ 
          "value".color(Color::Red),
          "[JSONB]".color(Color::Green)
        ],
        data_aligment: TableViewAlignment::Left
      },
      11
    ),
  ]
}