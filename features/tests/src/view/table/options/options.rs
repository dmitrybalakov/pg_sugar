use super::{TableViewOptionsData, TableViewOptionsGrid, TableViewOptionsHeader, TableViewOptionsSeparator};

pub struct TableViewOptions {
  pub data: TableViewOptionsData,
  pub grid: TableViewOptionsGrid,
  pub header: TableViewOptionsHeader,
  pub separtor: TableViewOptionsSeparator,
}

impl TableViewOptions {
  pub fn new() -> Self { 
    Self { 
      data: TableViewOptionsData::new(), 
      grid: TableViewOptionsGrid::new(), 
      header: TableViewOptionsHeader::new(), 
      separtor: TableViewOptionsSeparator::new() 
    } 
  }
}