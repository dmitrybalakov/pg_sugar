use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct FKey {
  pub schema: String,
  pub name: String,
}

impl FKey {
  pub fn id(&self) -> String {
    format!(r#""{}"."{}""#, self.schema, self.name)
  }  
}
