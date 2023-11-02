use colored::ColoredString;

use super::TableViewAlignment;

pub struct TableViewColumn {
  pub title: Vec<ColoredString>,
  pub name: String,
  pub data_aligment: TableViewAlignment
}