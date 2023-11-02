use postgres::{
  Client, Config, Error, Socket,
  tls::{MakeTlsConnect, TlsConnect}, 
};

use crate::tables::{Table, TablesSort};

pub trait Strategy {
  fn client<T>(&self, client: &mut Client, test_name: &str, config: &Config, tls: T) -> Result<Client, Error>
  where
    T: MakeTlsConnect<Socket> + 'static + Send,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send;
  fn release(&self, client: &mut Client, test_name: &str) -> Result<(), Error>;
  fn tables(&self, client: &mut Client, test_name: &str, sort: TablesSort) -> Result<Vec<Table>, Error>;
}