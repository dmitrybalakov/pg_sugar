use colored::ColoredString;
use serde_json::Value;

pub enum AssertTableDataRowAssertion<'a> {
  Value(&'a Value),
  Fn(&'a AssertTableDataRowAssertionFn)
}

pub type AssertTableDataRowAssertionFn = dyn Fn(&Value, &str) -> Result<(), Vec<ColoredString>>;