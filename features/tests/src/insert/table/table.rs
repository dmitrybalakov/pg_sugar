use postgres::{Client, Error};

use crate::tables::Table;

pub trait InsertTable {
  fn execute(&self, client: &mut Client, table: &Table) -> Result<(), Error>;
}