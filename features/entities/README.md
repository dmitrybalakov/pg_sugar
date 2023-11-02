# Entities

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, FromRow, Entity)]
#[entity(
  table_name = r#""Books""#, 
  methods = select, select_one
)]
pub struct Book {
  pub year: i32,
  pub isbn: String,
  pub author: String,
  pub title: String,
  pub description: String,
  #[entity(
    column = Publisher::select_by_id(column!("publisher")),
    value = entity.publisher.id
  )]
  pub publisher: Publisher
}

impl Book {
  pub fn select_by_isbn<T: Into<Source<&str>>>(isbn: T) -> QueryOne<Book> { 
    Self::select_one().mutate(move |mut query| {
      query
        .where_and(format!(
          "isbn = {}", 
          isbn.into().apply(&mut query)
        ))
    })
  }

  #[query_args()]
  pub fn select_by_isbn_with_macros(isbn: T) -> QueryOne<Book> { 
    Self::select_one().mutate(move |mut query| {
      query
        .where_and(format!(
          "isbn = {}", 
          isbn.into().apply(&mut query)
        ))
    })
  }

  pub fn select_by_author<T: Into<Source<&str>>>(author: T) -> Query<Book> { 
    Self::select().mutate(move |mut query| {
      query
        .where_and(format!(
          "author = {}", 
          isbn.into().apply(&mut query)
        ))
        .order_by("year")
        .order_by("title")
    })
  }
}
```