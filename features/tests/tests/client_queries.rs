mod prepare;

use pg_sugar_tests::{TestClient, strategies::SchemasStrategy};
use postgres::{NoTls, Error};

use prepare::{CONNECTION_STRING, Book};

#[test]
fn client_queries_execute() -> Result<(), Error> {
  let mut client = prepare("client_queries_execute")?;

  assert_eq!(
    client.execute(
      r#"UPDATE "Books" SET author = $2 WHERE author = $1"#, 
      &[ &"J. R. R. Tolkien", &"Tolkien J. R. R." ]
    )?,
    2
  );

  client.release()
}

#[test]
fn client_queries_query() -> Result<(), Error> {
  let mut client = prepare("client_queries_query")?;

  assert_eq!(
    client.query(r#"SELECT * FROM "Books""#, &[ ])
      .map(|x| x.iter().map(|row| row.into()).collect::<Vec<Book>>())?
      .sort(),
    Book::books().sort()
  );

  client.release()
}

#[test]
fn client_queries_query_one() -> Result<(), Error> {
  let mut client = prepare("client_queries_query_one")?;
  let hobbit = Book::hobbit();

  let hobbit_from_db: Book = client.query_one(
    r#"SELECT * FROM "Books" WHERE isbn = $1"#, 
    &[ &hobbit.isbn ]
  ).map(|x| (&x).into())?;

  assert_eq!(hobbit_from_db, hobbit);

  client.release()
}

fn prepare(test_name: &str) -> Result<TestClient<SchemasStrategy>, Error> {
  TestClient::schemas(&format!("sugar_tests_{}", test_name), CONNECTION_STRING, NoTls)
    .and_then(|mut x| Book::migrate(&mut x).map(|_| x))
    .and_then(|mut x| Book::insert_all(&mut x, &Book::books()).map(|_| x))
}