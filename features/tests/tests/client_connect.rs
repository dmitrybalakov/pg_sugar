mod prepare;

use pg_sugar_tests::TestClient;
use postgres::{NoTls, Error};

use prepare::CONNECTION_STRING;

#[test]
fn client_connect_databases() -> Result<(), Error> {
  let test_name = "sugar_tests_client_connect_databases";
  let mut client = TestClient::databases(test_name, CONNECTION_STRING, NoTls)?;

  assert_eq!(
    client.main_client.query(
      "SELECT datname FROM pg_database WHERE datname = $1", 
      &[ &test_name ]
    )?.len(),
    1
  );

  client.release()
}

#[test]
fn client_connect_schemas() -> Result<(), Error> {
  let test_name = "sugar_tests_client_connect_schemas";
  let mut client = TestClient::schemas(test_name, CONNECTION_STRING, NoTls)?;

  assert_eq!(
    client.main_client.query(
      "SELECT schema_name FROM information_schema.schemata WHERE schema_name = $1", 
      &[ &test_name ]
    )?.len(),
    1
  );

  client.release()
}

#[test]
#[should_panic]
fn client_connect_invalid_connection_string() {
  TestClient::schemas(
    "sugar_tests_client_connect_invalid_connection_string", 
    "invalid connection string", 
    NoTls
  ).unwrap();
}