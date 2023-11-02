use postgres::{
  Client, Config, Error, Socket,
  tls::{MakeTlsConnect, TlsConnect}
};

use crate::{
  strategies::Strategy,
  tables::{Table, TablesQuery, TablesQueryPopulate, TablesSort},
  TestClient, 
};

pub struct SchemasStrategy { }

impl Strategy for SchemasStrategy {
  fn client<T>(&self, client: &mut Client, test_name: &str, config: &Config, tls: T) -> Result<Client, Error>
  where
    T: postgres::tls::MakeTlsConnect<postgres::Socket> + 'static + Send,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as postgres::tls::TlsConnect<postgres::Socket>>::Future: Send 
  {
    client.execute(&format!(r#"DROP SCHEMA IF EXISTS "{}" CASCADE"#, test_name), &[ ])?;
    client.execute(&format!(r#"CREATE SCHEMA "{}""#, test_name), &[ ])?;

    let mut client = config.clone().connect(tls)?;
    client.execute(&format!("SET SEARCH_PATH = {};", test_name), &[]).unwrap();

    Ok(client)
  }

  fn release(&self, client: &mut Client, test_name: &str) -> Result<(), Error> {
    client.execute(&format!(r#"DROP SCHEMA IF EXISTS "{}" CASCADE"#, test_name), &[ ])
      .map(|_| ())
  }

  fn tables(&self, client: &mut Client, test_name: &str, sort: TablesSort) -> Result<Vec<Table>, Error> {
    TablesQuery {
      populate: TablesQueryPopulate {
        columns: true,
        primary_key: true
      },
      sort,
      schema: Some(test_name.into()),
    }.select(client)
  }
}

impl TestClient<SchemasStrategy> {
  pub fn schemas<T>(test_name: &str, params: &str, tls_mode: T) -> Result<TestClient<SchemasStrategy>, Error> 
  where
    T: MakeTlsConnect<Socket> + 'static + Send + Clone,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send
  {
    TestClient::connect(SchemasStrategy { }, test_name, params, tls_mode)
  }
}