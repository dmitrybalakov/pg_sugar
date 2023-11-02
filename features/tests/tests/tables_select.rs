mod prepare;

use std::collections::HashSet;

use pg_sugar_tests::{
  strategies::DatabasesStrategy,
  tables::{Table, TablesQuery, TablesQueryPopulate, TablesSort, Column, FKey},
  TestClient, 
};
use postgres::{NoTls, Error};

use prepare::{CONNECTION_STRING, DAGTables};

#[test]
fn table_select_without_schema_to_insert() -> Result<(), Error> {
  let mut client = prepare(
    "without_schema_to_insert", 
    vec![ DAGTables::create_ab("ab"), DAGTables::create_c("c") ]
  )?;

  assert_eq!(
    TablesQuery { 
      populate: DEFAULT_POPULATE,
      sort: TablesSort::ToInsert, 
      schema: None, 
    }.select(&mut client.client)?,
    tables(vec![ "a", "b", "c" ], &DEFAULT_POPULATE)
  );
  
  client.release()
}

#[test]
fn table_select_without_schema_to_delete() -> Result<(), Error> {
  let mut client = prepare(
    "without_schema_to_delete", 
    vec![ DAGTables::create_ab("ab"), DAGTables::create_c("c") ]
  )?;

  assert_eq!(
    TablesQuery { 
      populate: DEFAULT_POPULATE,
      sort: TablesSort::ToDelete, 
      schema: None, 
    }.select(&mut client.client)?,
    tables(vec![ "b", "c", "a" ], &DEFAULT_POPULATE)
  );
  
  client.release()
}

#[test]
fn table_select_with_schema_to_insert() -> Result<(), Error> {
  let mut client = prepare(
    "with_schema_to_insert", 
    vec![ DAGTables::create_ab("ab"), DAGTables::create_c("c") ]
  )?;

  assert_eq!(
    TablesQuery { 
      populate: DEFAULT_POPULATE,
      sort: TablesSort::ToInsert, 
      schema: Some("ab".into()), 
    }.select(&mut client.client)?,
    tables(vec![ "a", "b" ], &DEFAULT_POPULATE)
  );
  
  client.release()
}

#[test]
fn table_select_with_schema_to_delete() -> Result<(), Error> {
  let mut client = prepare(
    "with_schema_to_delete", 
    vec![ DAGTables::create_ab("ab"), DAGTables::create_c("c") ]
  )?;

  assert_eq!(
    TablesQuery { 
      populate: DEFAULT_POPULATE,
      sort: TablesSort::ToDelete, 
      schema: Some("ab".into()), 
    }.select(&mut client.client)?,
    tables(vec![ "b", "a" ], &DEFAULT_POPULATE)
  );
  
  client.release()
}

#[test]
fn table_select_without_populations() -> Result<(), Error> {
  let mut client = prepare(
    "table_select_without_populations", 
    vec![ DAGTables::create_ab("ab") ]
  )?;
  let populate = TablesQueryPopulate { columns: false, primary_key: false };

  assert_eq!(
    TablesQuery { 
      populate: populate.clone(),
      sort: TablesSort::ToInsert, 
      schema: Some("ab".into()), 
    }.select(&mut client.client)?,
    tables(vec![ "a", "b" ], &populate)
  );
  
  client.release()
}

#[test]
fn table_select_populate_all() -> Result<(), Error> {
  let mut client = prepare(
    "table_select_populate_all", 
    vec![ DAGTables::create_ab("ab") ]
  )?;
  let populate = TablesQueryPopulate { columns: true, primary_key: true };

  assert_eq!(
    TablesQuery { 
      populate: populate.clone(),
      sort: TablesSort::ToInsert, 
      schema: Some("ab".into()), 
    }.select(&mut client.client)?,
    tables(vec![ "a", "b" ], &populate)
  );
  
  client.release()
}


#[test]
#[should_panic]
fn table_select_with_ring0() {
  let mut client = prepare("with_ring0", vec![ DAGTables::create_ring("ring", 0) ]).unwrap();

  TablesQuery { 
    populate: DEFAULT_POPULATE,
    sort: TablesSort::ToInsert, 
    schema: Some("ring".into()), 
  }.select(&mut client.client).unwrap();
}

#[test]
#[should_panic]
fn table_select_with_ring5() {
  let mut client = prepare("with_ring5", vec![ DAGTables::create_ring("ring", 5) ]).unwrap();

  TablesQuery { 
    populate: DEFAULT_POPULATE,
    sort: TablesSort::ToInsert, 
    schema: None, 
  }.select(&mut client.client).unwrap();
}

fn prepare (test_name: &str, dags: Vec<DAGTables>) -> Result<TestClient<DatabasesStrategy>, Error> {
  TestClient::databases(
    &format!("sugar_tests_table_select_{}", test_name),
    CONNECTION_STRING, 
    NoTls
  )
    .and_then(|mut x| {
      for dag in dags.iter() {
        dag.migrate(&mut x.client)?;
      }

      Ok(x)
    })
}

fn tables(prefix: Vec<&str>, populate: &TablesQueryPopulate) -> Vec<Table> {
  let mut result = vec![];

  for p in prefix.into_iter() {
    if p == "a" {
      result.push(Table { 
        schema: "ab".into(), 
        name: "A0".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
      result.push(Table { 
        schema: "ab".into(), 
        name: "A1".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
      result.push(Table { 
        schema: "ab".into(), 
        name: "A2".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
      result.push(Table { 
        schema: "ab".into(), 
        name: "A3".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
    }

    if p == "b" {
      result.push(Table { 
        schema: "ab".into(), 
        name: "B0".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "ref_A0".into(), ty: "character varying".into() },
          Column { name: "ref_A2".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![ 
          FKey { schema: "ab".into(), name: "A0".into() },
          FKey { schema: "ab".into(), name: "A2".into() },
        ],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
      result.push(Table { 
        schema: "ab".into(), 
        name: "B1".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "ref_A0".into(), ty: "character varying".into() },
          Column { name: "ref_A1".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![ 
          FKey { schema: "ab".into(), name: "A0".into() },
          FKey { schema: "ab".into(), name: "A1".into() },
        ],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
      result.push(Table {
        schema: "ab".into(), 
        name: "B2".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "ref_A1".into(), ty: "character varying".into() },
          Column { name: "ref_A3".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![ 
          FKey { schema: "ab".into(), name: "A1".into() },
          FKey { schema: "ab".into(), name: "A3".into() },
        ],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
    }

    if p == "c" {
      result.push(Table {
        schema: "c".into(), 
        name: "C".into(), 
        columns: vec![
          Column { name: "id".into(), ty: "character varying".into() },
          Column { name: "title".into(), ty: "character varying".into() },
        ],
        foreign_keys: vec![ ],
        primary_key: vec![ "id".into() ].into_iter().collect()
      });
    }
  }

  result
    .into_iter()
    .map(|mut x| {
      if !populate.columns { x.columns = vec![]; }
      if !populate.primary_key { x.primary_key = HashSet::new(); }

      x
    })
    .collect()
}

const DEFAULT_POPULATE: TablesQueryPopulate = TablesQueryPopulate {
  columns: true,
  primary_key: true
};