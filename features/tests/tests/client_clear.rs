mod prepare;

use pg_sugar_tests::{TestClient, strategies::{Strategy, DatabasesStrategy}};
use postgres::{NoTls, Error};

use prepare::{CONNECTION_STRING, DAGTables};

use crate::prepare::Count;

#[test]
fn client_clear_only_schema() -> Result<(), Error> {
  let mut client = prepare("sugar_tests_client_clear_only_schema")?;
  let mut schema_client = TestClient::schemas(
    "ab", 
    "postgresql://postgres:123456@localhost:25432/sugar_tests_client_clear_only_schema", 
    NoTls
  )?;
  
  DAGTables::create_ab("ab").migrate(&mut client.client)?;
  schema_client.clear()?;

  assert_eq!(
    count(
      &mut client, 
      vec![ 
        r#""ab"."A0""#, r#""ab"."A1""#, r#""ab"."A2""#, r#""ab"."A3""#,
        r#""ab"."B0""#, r#""ab"."B1""#, r#""ab"."B2""#
      ]
    )?, 
    0
  );
  assert_eq!(count(&mut client, vec![ r#""c"."C""# ])?, 3);

  schema_client.release()?;
  client.release()
}

#[test]
fn client_clear_database() -> Result<(), Error> {
  let mut client = prepare("database")?;

  client.clear()?;

  assert_eq!(
    count(
      &mut client, 
      vec![ 
        r#""ab"."A0""#, r#""ab"."A1""#, r#""ab"."A2""#, r#""ab"."A3""#,
        r#""ab"."B0""#, r#""ab"."B1""#, r#""ab"."B2""#
      ]
    )?, 
    0
  );
  assert_eq!(count(&mut client, vec![ r#""c"."C""# ])?, 0);

  client.release()
}

fn prepare(test_name: &str) -> Result<TestClient<DatabasesStrategy>, Error> {
  TestClient::databases(
    test_name, 
    CONNECTION_STRING, 
    NoTls
  )
    .and_then(|mut x| {
      for dag in [ DAGTables::create_ab("ab"), DAGTables::create_c("c") ].iter() {
        dag.migrate(&mut x.client)?;
      }

      Ok(x)
    })
    .and_then(|mut x| {
      DAGTables::fill_ab(&mut x.client, "ab")?;
      DAGTables::fill_c(&mut x.client, "c")?;
      
      Ok(x)
    })
}

fn count<T: Strategy>(client: &mut TestClient<T>, tables: Vec<&str>) -> Result<i64, Error> {
  let r: Count = client.query_one(
    &format!(
      r#"
        SELECT 0{}
          AS count
      "#, 
      tables
        .iter()
        .map(|x| format!(
          "\n        + (SELECT count(*) FROM {})", x)
        )
        .collect::<Vec<String>>()
        .join("")
    ),
    &[]
  )?.into();

  Ok(r.count)
}