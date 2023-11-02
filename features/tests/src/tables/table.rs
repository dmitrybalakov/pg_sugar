use std::collections::HashSet;

use pg_sugar_from_row::{FromRow, FromRows};

use super::{Column, FKey};

#[derive(Debug, PartialEq, FromRow, FromRows)]
pub struct Table {
  pub schema: String,
  pub name: String,
  #[from_json()]
  pub columns: Vec<Column>,
  #[from_json(column = "foreignKeys")]
  pub foreign_keys: Vec<FKey>,
  #[from_row(from = row.get::<&str, Vec<String>>("primaryKey").into_iter().collect())]
  pub primary_key: HashSet<String>
}

impl Table {
  pub fn id(&self) -> String {
    format!(r#""{}"."{}""#, self.schema, self.name)
  }

  pub fn identifiers(&self) -> Vec<String> {
    vec![
      format!(r#"{}"#, self.name),
      format!(r#""{}""#, self.name),
      format!(r#"{}.{}"#, self.schema, self.name),
      format!(r#"{}."{}""#, self.schema, self.name),
      format!(r#""{}".{}"#, self.schema, self.name),
      format!(r#""{}"."{}""#, self.schema, self.name),
    ]
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

use super::Table;

  #[test]
  fn table_identifiers() {
    let table = Table { 
      schema: "public".into(), 
      name: "Books".into(), 
      columns: vec![], 
      foreign_keys: vec![],
      primary_key: HashSet::new()
    };

    assert_eq!(
      table.identifiers(),
      vec![
        r#"Books"#.to_string(),
        r#""Books""#.to_string(),
        r#"public.Books"#.to_string(),
        r#"public."Books""#.to_string(),
        r#""public".Books"#.to_string(),
        r#""public"."Books""#.to_string(),
      ]
    )
  }
}