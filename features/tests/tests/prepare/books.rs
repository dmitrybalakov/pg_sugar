use postgres::Error;

use pg_sugar_from_row::FromRow;
use pg_sugar_tests::{TestClient, strategies::Strategy};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromRow)]
pub struct Book {
  pub isbn: String,
  pub author: Option<String>,
  pub title: String,
  pub description: String,
}

#[allow(dead_code)]
impl Book {
  pub fn migrate<S: Strategy>(client: &mut TestClient<S>) -> Result<(), Error> {
    client.execute(CREATE_TABLE_SQL, &[]).map(|_| ())
  }

  pub fn insert_all<S: Strategy>(client: &mut TestClient<S>, books: &Vec<Book>) -> Result<(), Error> {
    for book in books.iter() {
      client.execute(INSERT_SQL, &[ &book.isbn, &book.author, &book.title, &book.description ])?;
    }

    Ok(())
  }

  pub fn select_all<S: Strategy>(client: &mut TestClient<S>) -> Result<Vec<Book>, Error> {
    client.query(SELECT_SQL, &[])
      .map(|rows| rows.iter().map(|row| row.into()).collect())
  }
}

#[allow(dead_code)]
impl Book {
  pub fn books () -> Vec<Book> {
    vec! [ 
      Book::hobbit(), 
      Book::the_two_towers() 
    ]
  }

  pub fn hobbit() -> Book {
    Book { 
      isbn: "9780044403371".into(), 
      author: Some("J. R. R. Tolkien".into()), 
      title: "The Annotated Hobbit: The Hobbit, or, There and Back Again".into(), 
      description: "1st Edition".into() 
    }
  }

  pub fn the_two_towers() -> Book {
    Book { 
      isbn: "9780007136568".into(), 
      author: Some("J. R. R. Tolkien".into()), 
      title: "The Two Towers (The Lord of the Rings) (Vol 2)".into(), 
      description: "Publisher: HarperCollins (2002)".into() 
    }
  }
}

pub const CREATE_TABLE_SQL: &str = r#"
CREATE TABLE "Books" (
  isbn        VARCHAR,
  author      VARCHAR,
  title       VARCHAR,
  description VARCHAR
);
"#;
pub const INSERT_SQL: &str = r#"INSERT INTO "Books" SELECT $1, $2, $3, $4"#;
pub const SELECT_SQL: &str = r#"SELECT * FROM "Books" ORDER BY "isbn""#;
