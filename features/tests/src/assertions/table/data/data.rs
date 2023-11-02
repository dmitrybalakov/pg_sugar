use std::collections::{HashMap, HashSet};

use postgres::{Client, Error};
use serde_json::Value;

use crate::{assertions::AssertTable, tables::Table};

use super::AssertTableDataRow;

pub struct AssertTableData<'a> {
  pub order_by: Option<String>,
  pub wh: Option<String>,
  pub rows: Vec<AssertTableDataRow<'a>>
}

impl<'a> AssertTable for AssertTableData<'a> {
  fn execute(
    &self, 
    client: &mut Client, 
    table: &Table
  ) -> Result<Option<String>, Error> {
    let mut by_hash = HashMap::new();
    for (index,row) in self.rows.iter().enumerate() {
      if self.order_by.is_some() {
        by_hash.insert(index.to_string(), index);
        continue;
      }
      
      match row.hash(table) {
        Ok(hash) => { by_hash.insert(hash, index); },
        Err(error) => return Ok(Some(error)),
      }
    }

    let rows = client.query_one(
      &format!(
        r#"SELECT (SELECT array_to_json(array_agg(row)) FROM (SELECT * FROM {table}{wh}{order_by}) AS row) "rows""#, 
        table = table.id(), 
        wh = match &self.wh {
          Some(x) => format!(" WHERE {}", x),
          None => "".into()
        },
        order_by = match &self.order_by {
          Some(x) => format!(" ORDER BY {}", x),
          None => "".into()
        },
      ), 
      &[]
    )?.get::<&str, Vec<Value>>("rows");
    
    let mut errors = HashMap::new();
    let mut columns = HashSet::new();
    let mut unsetted_row = vec![];
    
    for (index,db_row) in rows.iter().enumerate() {
      let row_hash = match self.order_by {
        Some(_) => index.to_string(),
        None => match Self::hash(db_row, table) {
          Ok(hash) => hash,
          Err(x) => return Ok(Some(x)),
        },
      }; 

      match by_hash.get(&row_hash) {
        Some(assert_index) => {
          match self.rows[*assert_index].assert(db_row, table) {
            Ok(_) => {  },
            Err(e) => { 
              for column_name in e.keys() {
                columns.insert(column_name.clone());
              }

              errors.insert(assert_index, e);
            },
          }
        },
        None => { unsetted_row.push(db_row); },
      }
    }

    todo!()
  }
}

impl<'a> AssertTableData<'a> {
  fn hash(value: &Value, table: &Table) -> Result<String, String> {
    let value_obj = value.as_object().unwrap();

    let mut parts = vec![];
    for name in table.primary_key.iter() {
      match value_obj.get(name) {
        Some(x) => parts.push(x.to_string()),
        None => return Err(format!(r#"primary column "{}" not found in row"#, name)),
      }
    }

    Ok(parts.join("_"))
  }
}