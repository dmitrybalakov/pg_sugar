use pg_sugar_tests::{TestClient, strategies::DatabasesStrategy, assertions::{Assert, AssertTableCount}};
use postgres::{Error, NoTls};

use crate::prepare::{DAGTables, CONNECTION_STRING};

#[test]
pub fn client_assert_count() -> Result<(), Error> {
  let mut client = prepare("sugar_tests_client_assert_count")?;

  client.assert(Assert::new(vec![
    ("A0", &AssertTableCount { count: 2, wh: None }),
    ("A1", &AssertTableCount { count: 1, wh: None }),
    ("A2", &AssertTableCount { count: 1, wh: None }),
    ("A3", &AssertTableCount { count: 1, wh: None }),
    ("C", &AssertTableCount { count: 3, wh: None }),
  ]))?;

  client.release()
}

#[test]
pub fn client_assert_count_wh() -> Result<(), Error> {
  let mut client = prepare("sugar_tests_client_assert_count_wh")?;

  client.assert(Assert::new(vec![
    ("A0", &AssertTableCount { count: 1, wh: Some(r#""id" = 'A0_0'"#.into()) }),
  ]))?;

  client.release()
}

#[test]
#[should_panic = "2 [database rows count] != 22 [expected rows count]"]
pub fn client_assert_count_panic() {
  let mut client = prepare("sugar_tests_client_assert_count_panic").unwrap();

  client.assert(Assert::new(vec![
    ("A0", &AssertTableCount { count: 22, wh: None }),
  ])).unwrap();
}

pub fn prepare(test_name: &str) -> Result<TestClient<DatabasesStrategy>, Error> {
  TestClient::databases(
    test_name, 
    CONNECTION_STRING, 
    NoTls
  ).and_then(|mut x| {
    DAGTables::create_ab("public").migrate(&mut x.client)?;
    DAGTables::create_c("public").migrate(&mut x.client)?;
    
    DAGTables::fill_ab(&mut x.client, "public")?;
    DAGTables::fill_c(&mut x.client, "public")?;

    Ok(x)
  })
}