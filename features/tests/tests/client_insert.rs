mod prepare;

use std::collections::HashMap;

use pg_sugar_tests::{
  strategies::SchemasStrategy, 
  insert::{
    table::{InsertTableJson, InsertTable, InsertTableValue, InsertTableColumnValue, InsertTableValues},
    Insert
  },
  TestClient, 
};
use postgres::{NoTls, Error};

use postgres_types::ToSql;
use prepare::{CONNECTION_STRING, Book};
use serde_json::json;

#[test]
fn client_insert_json() -> Result<(), Error> {
  let mut client = prepare("client_insert_json")?;
  let hobbit = Book::hobbit();

  let books: &dyn InsertTable = &InsertTableJson {
    data: json!([
      { "isbn": "0", "title": "Nullable column", "description": "Desc" },
      { "isbn": hobbit.isbn, "author": hobbit.author, "title": hobbit.title, "description": hobbit.description },
    ])
  };

  client.insert(Insert { tables: HashMap::from([ (r#""Books""#.into(), books) ]) })?;

  assert_eq!(
    Book::select_all(&mut client)?,
    vec![
      Book { isbn: "0".into(), author: None, title: "Nullable column".into(), description: "Desc".into() },
      hobbit
    ]
  );

  client.release()
}

#[test]
fn client_insert_values() -> Result<(), Error> {
  let mut client = prepare("client_insert_rows")?;
  let hobbit = Book::hobbit();

  let books: &dyn InsertTable = &InsertTableValues {
    values: vec![
      InsertTableValue { 
        columns: HashMap::from([ 
          ("isbn".into(), InsertTableColumnValue::Sql("'0'")),
          ("author".into(), InsertTableColumnValue::Null),
          ("title".into(), InsertTableColumnValue::Value(&"DataRowValue::Null")),
          ("description".into(), InsertTableColumnValue::SqlFn(&|params: &mut Vec<&(dyn ToSql + Sync)>| {
            params.push(&"Desc");
            format!("${}::VARCHAR", params.len())
          })),
        ]) 
      },
      InsertTableValue { 
        columns: HashMap::from([ 
          ("isbn".into(), InsertTableColumnValue::Value(&hobbit.isbn)),
          ("author".into(), InsertTableColumnValue::Value(&hobbit.author)),
          ("title".into(), InsertTableColumnValue::Value(&hobbit.title)),
          ("description".into(), InsertTableColumnValue::Value(&hobbit.description)),
        ]) 
      },
    ]
};

  client.insert(Insert { tables: HashMap::from([ (r#""Books""#.into(), books) ]) })?;

  assert_eq!(
    Book::select_all(&mut client)?,
    vec![
      Book { isbn: "0".into(), author: None, title: "DataRowValue::Null".into(), description: "Desc".into() },
      hobbit
    ]
  );

  client.release()
}

#[test]
#[should_panic]
fn client_insert_table_not_found() {
  let mut client = prepare("client_insert_table_not_found").unwrap();
  let hobbit = Book::hobbit();

  let books: &dyn InsertTable = &InsertTableJson {
    data: json!([
      { "isbn": "0", "title": "Nullable column", "description": "Desc" },
      { "isbn": hobbit.isbn, "author": hobbit.author, "title": hobbit.title, "description": hobbit.description },
    ])
  };

  client.insert(Insert { tables: HashMap::from([ (r#""BeBooks""#.into(), books) ]) }).unwrap();
}

fn prepare(test_name: &str) -> Result<TestClient<SchemasStrategy>, Error> {
  TestClient::schemas(&format!("sugar_tests_{}", test_name), CONNECTION_STRING, NoTls)
    .and_then(|mut x| Book::migrate(&mut x).map(|_| x))
}