use std::collections::HashMap;

use colored::{ColoredString, Color, Colorize};
use serde_json::Value;

use crate::tables::Table;
use super::AssertTableDataRowAssertion;

pub struct AssertTableDataRow<'a> {
  pub assertions: HashMap<String, AssertTableDataRowAssertion<'a>>   
}

impl<'a> AssertTableDataRow<'a> {
  pub fn hash(&self, table: &Table) -> Result<String, String> {
    let mut parts = vec![];
    for name in table.primary_key.iter() {
      match self.assertions.get(name) {
        Some(assertion) => {
          match assertion {
            AssertTableDataRowAssertion::Value(x) => { parts.push(x.to_string()); },
            AssertTableDataRowAssertion::Fn(_) => {
              return Err(format!(
                r#"primary key column "{}" must be specified"#, 
                name
              ))
            },
          }
        },
        None => return Err(format!(r#"primary key column "{}" not found"#, name)),
      }
    }

    Ok(parts.join("_"))
  }

  pub fn assert(&self, row: &Value, table: &Table) -> Result<(), HashMap<String, [Vec<ColoredString>; 2]>> {
    let mut errors = HashMap::new();

    for column in table.columns.iter() {
      match self.assertions.get(&column.name) {
        Some(column_assertion) => {
          match &column_assertion {
            AssertTableDataRowAssertion::Value(v) => {
              todo!();
            },
            AssertTableDataRowAssertion::Fn(assert_fn) => {
              if let Err(e) = assert_fn(row, &column.name) {
                errors.insert(column.name.clone(), e);
              }
            },
          }
        },
        None => { 
          errors.insert(
            column.name.clone(), 
            vec![ "undefined".into() ]
          ); 
        },
      };
    }

    match errors.len() { 0 => Ok(()), _ => Err(errors) }
  }

  fn populate_value(json_value: &Value, color: Color) -> Vec<ColoredString> {
    match json_value {
      Value::Null => vec![ "null".color(color) ],
      Value::Bool(value) => vec![ value.to_string().as_str().color(color) ],
      Value::Number(value) => vec![ value.to_string().as_str().color(color) ],
      Value::String(value) => vec![ value.as_str().color(color) ],
      Value::Array(value) => {
        match value.len() {
          0 => vec![ "[ ]".color(color) ],
          1 => { 
            let mut element = Self::populate_value(&value[0], color);
            match element.len() {
              0 => vec![ "[ no_json ]".color(color) ],
              1 => vec![ format!("[ {} ]", element[0]).as_str().color(color) ],
              _ => {
                let mut result = vec![ "[".color(color) ];


                result
              }
            }
          },
          _ => todo!()
        }
      },
      Value::Object(_) => todo!(),
    }
  }
}