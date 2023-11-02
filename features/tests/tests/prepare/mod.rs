mod books;
mod dag;

pub use books::*;
pub use dag::*;

use pg_sugar_from_row::FromRow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Count {
  pub count: i64
}

#[allow(dead_code)]
impl Count {
  pub fn new(count: i64) -> Self {
    Self { count }
  }
}

pub const CONNECTION_STRING: &str = "postgresql://postgres:123456@localhost:25432/main";