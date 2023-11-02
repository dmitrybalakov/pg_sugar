use colored::Color;

pub struct TableViewOptionsGrid {
  pub color: Color,
  pub horizontal_line: char,
  pub vertical_line: char,
  pub top_left: char,
  pub top_separator: char,
  pub top_right: char,
  pub middle_left: char,
  pub middle_separator: char,
  pub middle_right: char,
  pub bottom_left: char,
  pub bottom_separator: char,
  pub bottom_right: char,
}

impl TableViewOptionsGrid {
  pub fn new() -> Self {
    Self {
      color: Color::White,
      horizontal_line: '═',
      vertical_line: '║',
      top_left: '╔',
      top_separator: '╦',
      top_right: '╗',
      middle_left: '╠',
      middle_separator: '╬',
      middle_right: '╣',
      bottom_left: '╚',
      bottom_separator: '╩',
      bottom_right: '╝',
    }
  }
}