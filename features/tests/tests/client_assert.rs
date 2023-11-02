mod assertions;
mod prepare;

use pg_sugar_tests::{
  assertions::{Assert, AssertTable},
  strategies::DatabasesStrategy, 
  tables::Table, 
  TestClient, 
};
use postgres::{NoTls, Error, Client};

use prepare::{CONNECTION_STRING, DAGTables};

#[test]
fn client_assert_main() -> Result<(), Error> {
  let mut client = prepare("sugar_tests_client_assert_main")?;

  client.assert(Assert::new(vec![
    ("A0", &TestAssert(None)),
    ("A1", &TestAssert(None)),
    ("A2", &TestAssert(None)),
  ]))?;

  client.release()
}

#[test]
#[should_panic= "\nDatabase assertion error:\n\nA1\n  Test error\n\nA2\n  Test error22\n  Another line\n\n"]
fn client_assert_panic() {
  let mut client = prepare("sugar_tests_client_assert_panic").unwrap();

  client.assert(Assert::new(vec![
    ("A0", &TestAssert(None)),
    ("A1", &TestAssert(Some("Test error".into()))),
    ("A2", &TestAssert(Some("Test error22\nAnother line".into()))),
  ])).unwrap();
}

#[test]
#[should_panic= "Some tables not found: A12"]
fn client_assert_table_not_found() {
  let mut client = prepare("sugar_tests_client_assert_table_not_found").unwrap();

  client.assert(Assert::new(vec![
    ("A0", &TestAssert(None)),
    ("A12", &TestAssert(None)),
  ])).unwrap();
}

struct TestAssert(Option<String>);

impl AssertTable for TestAssert {
  fn execute(&self, _: &mut Client, _: &Table) -> Result<Option<String>, Error> {
    Ok(self.0.clone())
  }
}

pub fn prepare(test_name: &str) -> Result<TestClient<DatabasesStrategy>, Error> {
  TestClient::databases(
    test_name, 
    CONNECTION_STRING, 
    NoTls
  ).and_then(|mut x| {
    DAGTables::create_ab("ab").migrate(&mut x.client)?;

    Ok(x)
  })
}