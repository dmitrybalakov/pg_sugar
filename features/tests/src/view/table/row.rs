use std::collections::HashMap;

use colored::{ColoredString, Colorize};

use super::{TableViewOptions, TableViewColumn, TableViewOptionsHeader, TableViewAlignment, TableViewOptionsGrid};

pub enum TableViewRow {
  Header,
  Data(HashMap<String, Vec<ColoredString>>),
  Separator,
  Bottom,
}

impl TableViewRow {
  pub fn len(&self, column: &TableViewColumn, options: &TableViewOptions) -> Option<usize> {
    match self {
      TableViewRow::Header => match &options.header {
        TableViewOptionsHeader::Simple => None,
        TableViewOptionsHeader::Columns(columns_options) => Some(
          columns_options.prefix.len() 
          + column.title.iter().fold(0, |agg, x| agg.max(x.len()))
          + columns_options.postfix.len()
        ),
      },
      TableViewRow::Data(x) => x
        .get(&column.name)
        .map(|x| 
          options.data.prefix.len() 
          + x.iter().fold(0, |agg, cs| agg.max(cs.len()))
          + options.data.postfix.len()
        ),
      TableViewRow::Separator => None,
      TableViewRow::Bottom => None,
    }
  }

  pub fn to_colored_string(
    &self, 
    columns: &Vec<(&TableViewColumn, usize)>, 
    options: &TableViewOptions
  ) -> ColoredString {
    match self {
      TableViewRow::Header => {
        let mut result = vec![
          format!(
            "{left}{columns}{right}",
            left = options.grid.top_left,
            columns = columns
              .iter()
              .map(
                |(_column, len)| 
                  (0..*len).map(|_| &options.grid.horizontal_line)
                  .collect::<String>()
              )
              .collect::<Vec<String>>()
              .join(&options.grid.top_separator.to_string()
              .color(options.grid.color)),
            right = options.grid.top_right,
          )
            .color(options.grid.color)
            .to_string()
        ];

        if let TableViewOptionsHeader::Columns(columns_options) = &options.header {
          result.push(Self::format_data(
            &columns
              .iter()
              .map(|(c, _)| &c.title)
              .collect(),
            &columns_options.prefix, 
            &columns_options.postfix, 
            Some(&columns_options.alignment),
            columns,
            &options.grid
          ));
          result.push(
            format!(
              "{left}{columns}{right}",
              left = options.grid.middle_left,
              columns = columns
                .iter()
                .map(|(_column, len)| (0..*len).map(|_| options.grid.horizontal_line).collect())
                .collect::<Vec<String>>()
                .join(&options.grid.middle_separator.to_string()),
              right = options.grid.middle_right,
            )
              .color(options.grid.color)
              .to_string()
          );
        }

        result.join("\n").as_str().into()
      },
      TableViewRow::Data(data) => {
        let empty = vec![];
        
        Self::format_data(
          &columns
            .iter()
            .map(|(c, _)| data.get(&c.name).unwrap_or(&empty))
            .collect(),
          &options.data.prefix, 
          &options.data.postfix, 
          None,
          columns,
          &options.grid
        ).as_str().into()
      },
      TableViewRow::Separator => {
        let len = columns.iter().fold(0, |a, (_, len)| a + len + 1) + 1;
        (0..len)
          .map(|_| '.')
          .collect::<String>()
          .color(options.separtor.color)
      },
      TableViewRow::Bottom => {
        format!(
          "{left}{columns}{right}",
          left = options.grid.bottom_left,
          columns = columns
            .iter()
            .map(|(_column, len)| (0..*len).map(|_| options.grid.horizontal_line).collect())
            .collect::<Vec<String>>()
            .join(&options.grid.bottom_separator.to_string()),
          right = options.grid.bottom_right,
        ).color(options.grid.color)
      }
    }
  }

  pub fn format_data(
    data: &Vec<&Vec<ColoredString>>,
    prefix: &ColoredString, 
    postfix: &ColoredString, 
    alignment: Option<&TableViewAlignment>,
    columns: &Vec<(&TableViewColumn, usize)>,
    grid_opt: &TableViewOptionsGrid,
  ) -> String {
    let rows_count = data.iter().fold(0, |a, value| a.max(value.len()));

    (0..rows_count)
      .map(|row_index| format!(
        "{}{}",
        data
          .iter()
          .enumerate()
          .map(|(index, values)| {
            let column = columns.get(index).unwrap();
            format!(
              "{}{}",
              grid_opt.vertical_line.to_string().color(grid_opt.color),    
              alignment.unwrap_or(&column.0.data_aligment).format(
                prefix, 
                values.get(row_index).unwrap_or(&"".into()), 
                postfix, 
                column.1
              ),
            )
          })
          .collect::<String>(),
        grid_opt.vertical_line.to_string().color(grid_opt.color)
      ))
      .collect::<Vec<String>>()
      .join("\n")
  }
}