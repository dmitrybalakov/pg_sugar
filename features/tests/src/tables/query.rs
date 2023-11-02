use postgres::{Client, Error};

use crate::tables::{Table, TablesSort};

pub struct TablesQuery { 
  pub populate: TablesQueryPopulate,
  pub sort: TablesSort,
  pub schema: Option<String>,
}

#[derive(Clone)]
pub struct TablesQueryPopulate {
  pub columns: bool,
  pub primary_key: bool,
}

impl TablesQuery {
  pub fn select(&self, client: &mut Client) -> Result<Vec<Table>, Error> {
    println!("SQL\n{}\n\n", self.query_sql());

    match &self.schema {
      Some(x) => client.query(&self.query_sql(), &[ &x ]),
      None => client.query(&self.query_sql(), &[ ])
    }.and_then(|x| Ok(self.sort.topological_sort(Table::from_rows(x)).unwrap()))
  }

  fn query_sql(&self) -> String {
    format!(
      r#"
        SELECT 
          t.table_schema "schema", 
          t.table_name "name", 
          {columns}
          {foreign_keys}
          {primary_key}
        FROM 
          information_schema.tables t
        WHERE 
          table_type = 'BASE TABLE'{wh_schema}
          AND NOT (t.table_schema = ANY('{{pg_catalog, information_schema}}'))
        GROUP BY t.table_schema, t.table_name
        ORDER BY t.table_schema, t.table_name
      "#,
      columns = match self.populate.columns {
        true => r#"(SELECT array_to_json(array_agg(row)) FROM (
            SELECT cs.column_name "name", cs.data_type "ty"
            FROM information_schema.columns cs
            WHERE 
              cs.table_schema = t.table_schema
              AND cs.table_name = t.table_name
            ORDER BY cs.column_name
          ) AS row) "columns","#,
        false => r#"'[]'::JSON columns,"#
      },
      foreign_keys = match self.sort {
        TablesSort::None => r#"'[]'::JSON "foreignKeys","#,
        _ => r#"COALESCE((SELECT array_to_json(array_agg(row)) FROM (
            SELECT ccu.table_schema::VARCHAR "schema", ccu.table_name::VARCHAR "name"
            FROM 
              information_schema.table_constraints tc 
              LEFT JOIN information_schema.constraint_column_usage ccu 
                ON ccu.constraint_name = tc.constraint_name
            WHERE 
              tc.table_schema = t.table_schema 
              AND tc.table_name = t.table_name
              AND tc.constraint_type = 'FOREIGN KEY'
            ORDER BY ccu.table_schema, ccu.table_name
          ) AS row), '[]'::JSON) "foreignKeys","#,
      },
      primary_key = match self.populate.primary_key {
        true => r#"COALESCE(ARRAY(
            SELECT ccu.column_name::VARCHAR
            FROM 
              information_schema.table_constraints tc 
              LEFT JOIN information_schema.constraint_column_usage ccu 
                ON ccu.constraint_name = tc.constraint_name
            WHERE 
              tc.table_schema = t.table_schema 
              AND tc.table_name = t.table_name
              AND tc.constraint_type = 'PRIMARY KEY'
            ORDER BY ccu.table_schema, ccu.table_name
          ), '{}'::VARCHAR[]) "primaryKey""#,
        false => r#"'{}'::VARCHAR[] "primaryKey""#
      },
      wh_schema = match &self.schema {
        Some(_) => "\n          AND t.table_schema = $1",
        None => "",
      },
    )
  }
}