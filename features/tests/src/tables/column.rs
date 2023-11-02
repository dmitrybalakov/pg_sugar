use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Column {
  pub name: String,
  pub ty: String
}