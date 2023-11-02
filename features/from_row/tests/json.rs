mod prepare;

use pg_sugar_from_row::FromRow;
use prepare::connect;
use serde::Deserialize;

#[test]
fn json_main() {
  let mut client = connect();

  assert_eq!(
    client.query(SQL, &[])
      .map(|x| x.iter().map(|row| row.into()).collect::<Vec<TestStruct>>())
      .unwrap(),
    vec![
      TestStruct { 
        json: JsonColumn { a: "aaa".into(), b: 0 }, 
        json_array: vec![ JsonColumn { a: "a".into(), b: 0 }, JsonColumn { a: "b".into(), b: 1 } ], 
        def: vec![], 
        nullable: Some(JsonColumn { a: "n".into(), b: 42 })
      },
      TestStruct { 
        json: JsonColumn { a: "aaa".into(), b: 0 }, 
        json_array: vec![ ], 
        def: vec![], 
        nullable: None
      }
    ]
  )
}

#[derive(Debug, PartialEq, FromRow)]
struct TestStruct {
  #[from_json(column = "json")]
  pub json: JsonColumn,
  #[from_json(column = "jsonArray")]
  pub json_array: Vec<JsonColumn>,
  #[from_json(column = "def", default = vec![])]
  pub def: Vec<JsonColumn>,
  #[from_nullable_json(column = "nullable")]
  pub nullable: Option<JsonColumn>
}

#[derive(Debug, PartialEq, Deserialize)]
struct JsonColumn {
  pub a: String,
  #[serde(rename(deserialize = "B"))]
  pub b: i32
}

const SQL: &str = r#"
SELECT * 
FROM (VALUES 
  (
    '{"a":"aaa","B":0}'::JSONB, 
    '[ {"a":"a","B":0}, {"a":"b","B":1} ]'::JSON,
    '[]'::JSON,
    '{"a":"n","B":42}'::JSONB
  ),
  (
    '{"a":"aaa","B":0}'::JSONB, 
    '[]'::JSON,
    NULL,
    NULL
  )
) AS t(
  "json", 
  "jsonArray",
  "def",
  "nullable"
)
"#;