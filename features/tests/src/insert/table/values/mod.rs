mod column_value;
mod value;

use postgres::{Client, Error};
use postgres_types::ToSql;

use crate::{
  insert::table::InsertTable,
  tables::Table,
};

pub use column_value::*;
pub use value::*;

pub struct InsertTableValues<'a> {
  pub values: Vec<InsertTableValue<'a>>
}

impl<'a> InsertTable for InsertTableValues<'a> {
  fn execute(&self, client: &mut Client, table: &Table) -> Result<(), Error> {
    let mut params = vec![];
    client.execute(
      &format!(
        r#"INSERT INTO {tn} ({cl}) VALUES {vl}"#,
        tn = table.id(),
        cl = table.columns.iter().map(|x| format!(r#""{}""#, x.name)).collect::<Vec<String>>().join(", "),
        vl = self.sql_values(table, &mut params)
      ), 
      &params
    ).map(|_| ())
  }
}

impl<'a> InsertTableValues<'a> {
  pub fn sql_values(
    &self, 
    table: &Table,
    params: &mut Vec<&'a (dyn ToSql + Sync)>,
  ) -> String {
    let mut result = vec![];
    
    for row in self.values.iter() {
      result.push(row.sql_values(table, params));
    }

    result.join(",")
  }
}