use postgres_types::ToSql;

pub enum InsertTableColumnValue<'a> {
  Null,
  Value(&'a (dyn ToSql + Sync)),
  Sql(&'a str),
  SqlFn(&'a dyn Fn(&mut Vec<&'a (dyn ToSql + Sync)>) -> String),
}

impl<'a> InsertTableColumnValue<'a> {
  pub fn sql(&self, params: &mut Vec<&'a (dyn ToSql + Sync)>, ty: &str) -> String {
    match self {
      InsertTableColumnValue::Null => format!("NULL"),
      InsertTableColumnValue::Value(x) => format!("${}::{}", { params.push(*x); params.len() }, ty),
      InsertTableColumnValue::Sql(sql) => sql.to_string(),
      InsertTableColumnValue::SqlFn(x) => x(params)
    }
  }
}