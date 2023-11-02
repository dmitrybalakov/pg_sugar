mod prepare;

use pg_sugar_from_row::FromRow;
use prepare::connect;

#[derive(Debug, PartialEq, FromRow)]
struct TestStruct {
  pub opt: Option<String>
}

#[test]
fn option_main() {
  let mut client = connect();

  assert_eq!(
    client.query(SQL, &[])
      .map(|x| x.iter().map(|row| row.into()).collect::<Vec<TestStruct>>())
      .unwrap(),
    vec![
      TestStruct { opt: None },
      TestStruct { opt: Some("AAA".into()) }
    ]
  )
}

const SQL: &str = r#"
SELECT * 
FROM (VALUES (NULL), ('AAA')) AS t("opt")
"#;