use std::collections::HashMap;

use postgres::Error;

use crate::{
  strategies::Strategy, 
  tables::{TablesSort, Tables},
  TestClient, 
};

use super::AssertTable;

pub struct Assert<'a> {
  order: Vec<String>,
  tables: HashMap<String, &'a dyn AssertTable>,
}

impl<'a> Assert<'a> {
  pub fn new(tables: Vec<(&str, &'a dyn AssertTable)>) -> Self {
    Self { 
      order: tables.iter().map(|x| x.0.into()).collect(),
      tables: tables
        .into_iter()
        .fold(HashMap::new(), |mut agg, (table_id, table)| {
          agg.insert(table_id.into(), table);
          agg
        }),
    }
  }

  pub fn execute<T: Strategy>(&self, client: &mut TestClient<T>) -> Result<Option<String>, Error> {
    let tables = Tables::new(self.tables.keys().collect());
    let db_tables = client.strategy.tables(
      &mut client.client, 
      &client.test_name, 
      TablesSort::None
    )?;

    tables.validate(&db_tables);

    let mut errors = HashMap::new();
    for table in db_tables.iter() {
      let normalized_id = tables.normalize_id(table);
      if let Some(data) = self.tables.get(&normalized_id) {
        if let Some(err) = data.execute(&mut client.client, &table)? {
          let error = format!(
            "{}\n{}", 
            normalized_id, 
            err
              .split("\n")
              .map(|x| format!("  {}", x))
              .collect::<Vec<String>>()
              .join("\n")
          );

          errors.insert(normalized_id, error);
        }
      }
    }

    Ok(
      match errors.len() {
        0 => None,
        _ => Some(
          self.order
            .iter()
            .fold(vec![], |mut agg, x| {
              if let Some(error) = errors.get(x) {
                agg.push(error.as_str());
              }

              agg
            })
            .join("\n\n")
        )
      }
    )
  }
}