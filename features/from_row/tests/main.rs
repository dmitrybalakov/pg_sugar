mod prepare;

use pg_sugar_from_row::{FromRow, FromRows};
use prepare::connect;

#[derive(Debug, PartialEq, FromRow, FromRows)]
struct TestStruct {
  pub main: String,
  #[from_row(column = "columnName")]
  pub column: bool,
  #[from_row(default = 42)]
  pub def: i32,
  #[from_row(from = vec![ "A".into() ])]
  pub from: Vec<String>,
  #[from_row(column = "cad", default = "<NO CAD>".into())]
  pub column_and_default: String
}

#[test]
fn main_from_row() {
  let mut client = connect();

  assert_eq!(
    client.query(SQL, &[]).map(TestStruct::from_rows).unwrap(),
    vec![
      TestStruct { 
        main: "main vlue:0".into(), 
        column: true, 
        def: 12, 
        from: vec![ "A".into() ], 
        column_and_default: "cad value".into() 
      },
      TestStruct { 
        main: "main vlue:1".into(), 
        column: false, 
        def: 42, 
        from: vec![ "A".into() ], 
        column_and_default: "<NO CAD>".into() 
      }
    ]
  )
}

const SQL: &str = r#"
SELECT * 
FROM (VALUES 
  ('main vlue:0', true, 12, 'ov', 'Poe,Tolkien', 'cad value'),
  ('main vlue:1', false, NULL, NULL, NULL, NULL)  
) AS t("main", "columnName", "def", "opt", "from", "cad")
"#;