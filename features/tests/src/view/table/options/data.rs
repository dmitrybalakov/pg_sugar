use colored::ColoredString;

pub struct TableViewOptionsData {
  pub prefix: ColoredString,
  pub postfix: ColoredString,
}

impl TableViewOptionsData {
  pub fn new() -> Self {
    Self { prefix: " ".into(), postfix: " ".into() }
  }
}
