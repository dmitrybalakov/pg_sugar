use postgres::{Client, Error};

use crate::tables::Table;

pub trait AssertTable {
  fn execute(&self, client: &mut Client, table: &Table) -> Result<Option<String>, Error>;
}