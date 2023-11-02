use postgres::{
  Client, Config, Error, Socket,
  tls::{MakeTlsConnect, TlsConnect}
};

use crate::{
  strategies::Strategy,
  tables::{Table, TablesQuery, TablesQueryPopulate, TablesSort},
  TestClient, 
};

pub struct DatabasesStrategy { }

impl Strategy for DatabasesStrategy {
  fn client<T>(&self, client: &mut Client, test_name: &str, config: &Config, tls: T) -> Result<Client, Error>
  where
    T: postgres::tls::MakeTlsConnect<postgres::Socket> + 'static + Send,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as postgres::tls::TlsConnect<postgres::Socket>>::Future: Send 
  {
    client.execute(&format!(r#"DROP DATABASE IF EXISTS "{}""#, test_name), &[ ])?;
    client.execute(&format!(r#"CREATE DATABASE "{}""#, test_name), &[ ])?;

    config.clone().dbname(&test_name).connect(tls)
  }

  fn release(&self, client: &mut Client, test_name: &str) -> Result<(), Error> {
    client.execute(&format!(r#"DROP DATABASE IF EXISTS "{}""#, test_name), &[ ])
      .map(|_| ())
  }

  fn tables(&self, client: &mut Client, _test_name: &str, sort: TablesSort) -> Result<Vec<Table>, Error> {
    TablesQuery {
      populate: TablesQueryPopulate {
        columns: true,
        primary_key: true
      },
      sort,
      schema: None,
    }.select(client)
  }
}

impl TestClient<DatabasesStrategy> {
  pub fn databases<T>(test_name: &str, params: &str, tls_mode: T) -> Result<TestClient<DatabasesStrategy>, Error> 
  where
    T: MakeTlsConnect<Socket> + 'static + Send + Clone,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send
  {
    TestClient::connect(DatabasesStrategy { }, test_name, params, tls_mode)
  }
}