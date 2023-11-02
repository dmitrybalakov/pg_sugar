use postgres::{Client, Error};

use crate::{
  assertions::AssertTable,
  tables::Table
};

pub struct AssertTableCount {
  pub count: i64,
  pub wh: Option<String>,
}

impl AssertTable for AssertTableCount {
  fn execute(
    &self, 
    client: &mut Client, 
    table: &Table
  ) -> Result<Option<String>, Error> {
    let db_count: i64 = client
      .query_one(
        &format!(
          "SELECT count(*) count FROM {}{}", 
          table.id(),
          match &self.wh {
            Some(x) => format!(" WHERE {}", x),
            None => "".into(),
          }
        ), 
        &[]
      )?
      .get("count");

    match db_count == self.count {
      true => Ok(None),
      false => Ok(Some(format!(
        "{} [database rows count] != {} [expected rows count]", 
        db_count, 
        self.count
      )))
    }
  }
}