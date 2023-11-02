use super::{TableViewOptions, TableViewColumn, TableViewRow};

pub struct TableView {
  pub options: TableViewOptions,
  pub columns: Vec<TableViewColumn>,
  pub rows: Vec<TableViewRow>,
}

impl TableView {
  pub fn to_string(&self) -> String {
    let columns_with_len = self.columns
      .iter()
      .map(|column| (
        column, 
        self.rows.iter().fold(0, |a, x| a.max(x.len(column, &self.options).unwrap_or(0))))
      )
      .collect();

    self.rows
      .iter()
      .map(|x| x.to_colored_string(&columns_with_len, &self.options).to_string())
      .collect::<Vec<String>>()
      .join("\n")
  }
}
