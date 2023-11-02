use colored::Color;

pub struct TableViewOptionsSeparator {
  pub color: Color
}

impl TableViewOptionsSeparator {
  pub fn new() -> Self {
    Self { 
      color: Color::TrueColor { r: 180, g: 180, b: 180 } 
    }
  }
}