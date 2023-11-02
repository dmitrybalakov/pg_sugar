use std::collections::HashMap;

use postgres::{Client, Error};

pub struct DAGTables {
  schema: String,
  tables: Vec<String>,
  fkeys: HashMap<String, Vec<String>>
}

#[allow(dead_code)]
impl DAGTables {
  pub fn new(schema: &str) -> Self {
    DAGTables { schema: schema.into(), tables: vec![], fkeys: HashMap::new() }
  }

  pub fn table(&mut self, table_name: &str, fkeys: Vec<&str>) -> &mut Self {
    self.tables.push(table_name.into());
    self.fkeys.insert(table_name.into(), fkeys.into_iter().map(|x| x.into()).collect());

    self
  }

  pub fn migrate(&self, client: &mut Client) -> Result<(), Error> {
    client.execute(&format!(r#"CREATE SCHEMA "{}";"#, self.schema), &[]).unwrap_or(0);
    
    // Create tables
    for table_name in self.tables.iter() {
      client.execute(
        &format!(
          r#"CREATE TABLE {}."{}" ({}, PRIMARY KEY ("id"))"#,
          self.schema,
          table_name,
          {
            let mut columns = match self.fkeys.get(table_name) {
              Some(x) => x.iter().map(|x| format!(r#""ref_{}" VARCHAR"#, x)).collect(),
              None => vec![]
            };
            
            columns.push(r#""id" VARCHAR"#.into());
            columns.push(r#""title" VARCHAR"#.into());
            
            columns.join(", ")
          }
        ), 
        &[]
      )?;
    }

    // Create FKeys
    for (table_name, fkeys) in self.fkeys.iter() {
      for ftable in fkeys.iter() {
        client.execute(
          &format!(
            r#"ALTER TABLE {sc}."{tn}" ADD FOREIGN KEY ("ref_{ft}") REFERENCES {sc}."{ft}"(id);"#,
            sc = self.schema,
            tn = table_name,
            ft = ftable,
          ), 
          &[]
        )?;
      }
    }

    Ok(())
  }

  pub fn create_ab(schema: &str) -> Self {
    let mut dag = Self::new(schema);
    dag
      .table("A0", vec![])
      .table("A1", vec![])
      .table("A2", vec![])
      .table("A3", vec![])
      .table("B0", vec![ "A0", "A2" ])
      .table("B1", vec![ "A0", "A1" ])
      .table("B2", vec![ "A1", "A3" ]);
  
    dag
  }

  pub fn create_c(schema: &str) -> Self {
    let mut dag = Self::new(schema);
    dag.table("C", vec![]);

    dag
  }

  pub fn create_ring(schema: &str, deep: usize) -> Self {
    let mut dag = Self::new(schema);

    match deep {
      0 => { dag.table("R", vec![ "R" ]); },
      _ => {
        for i in 0..deep {
          dag.table(&format!("R{}", i), vec![ &format!("R{}", i + 1) ]);
        }

        dag.table(&format!("R{}", deep), vec![ "R0" ]);
      }
    }

    dag
  }

  pub fn fill_ab(client: &mut Client, prefix: &str) -> Result<(), Error> {
    client.execute(
      &format!(r#"INSERT INTO "{}"."A0" (id, title) VALUES ('A0_0', 'A0 - 0'), ('A0_1', 'A0 - 1')"#, prefix), 
      &[]
    )?;
    client.execute(
      &format!(r#"INSERT INTO "{}"."A1" (id, title) VALUES ('A1_0', 'A1 - 0')"#, prefix), 
      &[]
    )?;
    client.execute(
      &format!(r#"INSERT INTO "{}"."A2" (id, title) VALUES ('A2_0', 'A2 - 0')"#, prefix), 
      &[]
    )?;
    client.execute(
      &format!(r#"INSERT INTO "{}"."A3" (id, title) VALUES ('A3_0', 'A3 - 0')"#, prefix), 
      &[]
    )?;
    client.execute(
      &format!(r#"INSERT INTO "{}"."B1" (id, title, "ref_A0", "ref_A1") VALUES ('B1_0', 'B1 - 0', 'A0_0', 'A1_0')"#, prefix), 
      &[]
    )?;

    Ok(())
  }

  pub fn fill_c(client: &mut Client, prefix: &str) -> Result<(), Error> {
    client.execute(
      &format!(r#"INSERT INTO "{}"."C" (id, title) VALUES ('C0', 'C - 0')"#, prefix),
      &[]
    )?;
    client.execute(
      &format!(r#"INSERT INTO "{}"."C" (id, title) VALUES ('C1', 'C - 1')"#, prefix),
      &[]
    )?;
    client.execute(
      &format!(r#"INSERT INTO "{}"."C" (id, title) VALUES ('C2', 'C - 2')"#, prefix),
      &[]
    )?;

    Ok(())
  }
}