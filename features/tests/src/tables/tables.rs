use std::collections::HashSet;

use super::Table;

pub struct Tables<'a> (HashSet<&'a String>);

impl<'a> Tables<'a> {
  pub fn new(data: HashSet<&'a String>) -> Self {
    Self(data)
  }

  pub fn validate(&self, tables: &Vec<Table>) {
    let mut unprocessed = self.0.clone();

    for table in tables.iter() {
      for id in table.identifiers().iter() {
        unprocessed.remove(id);
      }
    }

    if unprocessed.len() != 0 {
      panic!(
        "Some tables not found: {}", 
        unprocessed
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<String>>()
          .join(", ")
      )
    }
  }

  pub fn normalize_id(&self, table: &Table) -> String {
    for id in table.identifiers().iter() {
      if self.0.contains(id) {
        return id.to_string()
      }
    }

    table.id()
  }
}