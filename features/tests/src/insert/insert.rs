use std::collections::HashMap;

use postgres::Error;

use crate::{
  strategies::Strategy, 
  tables::{TablesSort, Tables},
  insert::table::InsertTable,
  TestClient, 
};

pub struct Insert<'a> {
  pub tables: HashMap<String, &'a dyn InsertTable>
}

impl<'a> Insert<'a> {
  pub fn execute<T: Strategy>(&self, client: &mut TestClient<T>) -> Result<(), Error> {
    let tables = Tables::new(self.tables.keys().collect());
    let sorted_tables = client.strategy.tables(
      &mut client.client, 
      &client.test_name, 
      TablesSort::ToInsert
    )?;

    tables.validate(&sorted_tables);

    for table in sorted_tables.iter() {
      if let Some(data) = self.tables.get(&tables.normalize_id(table)) {
        data.execute(&mut client.client, &table)?; 
      }
    }

    Ok(())
  }
}