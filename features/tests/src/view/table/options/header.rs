use colored::{Color, ColoredString};

use crate::view::TableViewAlignment;

pub enum TableViewOptionsHeader {
  Simple,
  Columns(TableViewOptionsHeaderColumns)
}

pub struct TableViewOptionsHeaderColumns {
  pub color: Color,
  pub prefix: ColoredString,
  pub postfix: ColoredString,
  pub alignment: TableViewAlignment,
}

impl TableViewOptionsHeader {
  pub fn new() -> Self {
    Self::Columns(TableViewOptionsHeaderColumns::new())
  }
}

impl TableViewOptionsHeaderColumns {
  pub fn new() -> Self {
    Self {
      color: Color::White, 
      prefix: " ".into(), 
      postfix: " ".into(), 
      alignment: TableViewAlignment::Center
    }
  }
}