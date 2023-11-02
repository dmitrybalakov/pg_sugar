use std::collections::HashMap;

use postgres_types::ToSql;

use crate::tables::Table;

use super::InsertTableColumnValue;

pub struct InsertTableValue<'a> {
  pub columns: HashMap<String, InsertTableColumnValue<'a>>  
}

impl<'a> InsertTableValue<'a> {
  pub fn columns(&self) -> Vec<String> {
    self
      .columns
      .keys()
      .map(|x| x.into())
      .collect()
  }

  pub fn sql_values(
    &self, 
    table: &Table,
    params: &mut Vec<&'a (dyn ToSql + Sync)>, 
  ) -> String {
    let mut result = vec![];
    for column in table.columns.iter() {
      result.push(self.columns.get(&column.name).unwrap_or(&InsertTableColumnValue::Null).sql(params, &column.ty));
    }
    
    format!("({})", result.join(","))
  }
}