use postgres::{Client, Error};
use serde_json::Value;

use crate::tables::Table;

use super::InsertTable;

pub struct InsertTableJson {
  pub data: Value
}

impl InsertTable for InsertTableJson {
  fn execute(&self, client: &mut Client, table: &Table) -> Result<(), Error> {
    client.execute(
      &format!(
        r#"INSERT INTO {tn} SELECT * FROM json_populate_recordset(null::{tn}, $1::JSON)"#,
        tn = table.id()
      ), 
      & [ &self.data ]
    ).map(|_| ())
  }
}