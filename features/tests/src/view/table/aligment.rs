use colored::ColoredString;

pub enum TableViewAlignment {
  Left,
  Center,
  Right
}

impl TableViewAlignment {
  pub fn format(
    &self,
    prefix: &ColoredString,
    value: &ColoredString,
    postfix: &ColoredString,
    len: usize
  ) -> String {
    let delta = len - prefix.len() - value.len() - postfix.len();
    let left_delta = match self {
      TableViewAlignment::Left => 0,
      TableViewAlignment::Center => delta / 2,
      TableViewAlignment::Right => delta,
    };
    let right_delta = delta - left_delta;
    
    format!(
      "{}{}{}{}{}",
      prefix,
      (0..left_delta).map(|_| ' ').collect::<String>(),
      value,
      (0..right_delta).map(|_| ' ').collect::<String>(),
      postfix
    ).as_str().into()
  }
}