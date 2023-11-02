use postgres::{
  tls::{MakeTlsConnect, TlsConnect}, 
  Client, Error, Socket, ToStatement, Row, 
};
use postgres_types::ToSql;

use crate::{insert::Insert, strategies::Strategy, tables::TablesSort, assertions::Assert};

pub struct TestClient<S: Strategy + Sized> {
  pub test_name: String,
  pub main_client: Client,
  pub client: Client,
  pub strategy: S
}

impl<S: Strategy + Sized> TestClient<S> {
  pub fn connect<T>(strategy: S, test_name: &str, params: &str, tls_mode: T) -> Result<Self, Error>
  where
    T: MakeTlsConnect<Socket> + 'static + Send + Clone,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send
  {
    let config = params.parse()?;
    let mut main_client = Client::connect(params, tls_mode.clone())?;

    Ok(Self {
      client: strategy.client(&mut main_client, test_name, &config, tls_mode)?, 
      test_name: test_name.into(), 
      main_client, 
      strategy
    })
  }

  pub fn release(mut self) -> Result<(), Error> {
    self.client.close()?;

    self.strategy.release(&mut self.main_client, &self.test_name)?;
    Ok(())
  }

  pub fn assert(&mut self, assert: Assert) -> Result<(), Error> { 
    match assert.execute(self)? {
      Some(error) => panic!("\nDatabase assertion error:\n\n{}\n\n", error),
      None => Ok(()),
    }
  }

  pub fn insert(&mut self, insert: Insert) -> Result<(), Error> { 
    insert.execute(self)
  }

  pub fn clear(&mut self) -> Result<(), Error> {
    let tables = self.strategy.tables(&mut self.client, &self.test_name, TablesSort::ToDelete)?;
    
    for table in tables.iter() {
      println!("{}", table.id());
      self.execute(&format!(r#"DELETE FROM {}"#, table.id()), &[])?;
    }

    Ok(())
  }

  pub fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
  where
    T: ?Sized + ToStatement,
  {
    self.client.execute(query, params)
  }

  pub fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
  where
    T: ?Sized + ToStatement,
  {
    self.client.query(query, params)
  }

  pub fn query_one<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
  where
    T: ?Sized + ToStatement,
  {
    self.client.query_one(query, params)
  }
}