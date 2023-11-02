use colored::{Colorize, Color};
use pg_sugar_tests::view::TableViewAlignment;

#[test]
fn left_format() {
  assert_eq!(
    TableViewAlignment::Left.format(
      &"  ".color(Color::White), 
      &"abc".color(Color::Red), 
      &"  ".color(Color::White), 
      12
    ),
    format!(
      "{prefix}{left}{value}{right}{postfix}",
      prefix = "  ".color(Color::White),
      left = "",
      value = "abc".color(Color::Red),
      right = "     ",
      postfix = "  ".color(Color::White),
    )
  );
}

#[test]
fn center_format() {
  assert_eq!(
    TableViewAlignment::Center.format(
      &"  ".color(Color::White), 
      &"abc".color(Color::Red), 
      &"  ".color(Color::White), 
      12
    ),
    format!(
      "{prefix}{left}{value}{right}{postfix}",
      prefix = "  ".color(Color::White),
      left = "  ",
      value = "abc".color(Color::Red),
      right = "   ",
      postfix = "  ".color(Color::White),
    )
  );
  assert_eq!(
    TableViewAlignment::Center.format(
      &"  ".color(Color::White), 
      &"abc".color(Color::Red), 
      &"  ".color(Color::White), 
      11
    ),
    format!(
      "{prefix}{left}{value}{right}{postfix}",
      prefix = "  ".color(Color::White),
      left = "  ",
      value = "abc".color(Color::Red),
      right = "  ",
      postfix = "  ".color(Color::White),
    )
  );
}

#[test]
fn right_format() {
  assert_eq!(
    TableViewAlignment::Right.format(
      &"  ".color(Color::White), 
      &"abc".color(Color::Red), 
      &"  ".color(Color::White), 
      12
    ),
    format!(
      "{prefix}{left}{value}{right}{postfix}",
      prefix = "  ".color(Color::White),
      left = "     ",
      value = "abc".color(Color::Red),
      right = "",
      postfix = "  ".color(Color::White),
    )
  );
}