mod prepare;

use pg_sugar_tests::TestClient;
use postgres::{Client, NoTls, Error};

use prepare::CONNECTION_STRING;

#[test]
fn client_release_databases() -> Result<(), Error> {
  let test_name = "sugar_tests_client_release_databases";
  TestClient::databases(test_name, CONNECTION_STRING, NoTls)?.release()?;

  assert_eq!(
    pg_client()?.query(
      "SELECT datname FROM pg_database WHERE datname = $1", 
      &[ &test_name ]
    )?.len(),
    0
  );

  Ok(())  
}

#[test]
fn client_release_schemas() -> Result<(), Error> {
  let test_name = "sugar_tests_client_release_schemas";
  TestClient::schemas(test_name, CONNECTION_STRING, NoTls)?.release()?;

  assert_eq!(
    pg_client()?.query(
      "SELECT schema_name FROM information_schema.schemata WHERE schema_name = $1", 
      &[ &test_name ]
    )?.len(),
    0
  );

  Ok(())
}

fn pg_client() -> Result<Client, Error> { Client::connect(CONNECTION_STRING, NoTls) }