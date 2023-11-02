use std::collections::{HashSet, HashMap};

use super::table::Table;

#[derive(PartialEq)]
pub enum TablesSort {
  None,
  ToInsert,
  ToDelete,
}

impl TablesSort {
  // Uses Kahn's algorithm
  pub fn topological_sort(&self, tables: Vec<Table>) -> Result<Vec<Table>, String> {
    let mut tables_mut: Vec<Table> = tables.into_iter().map(|x| x).collect();

    if self == &TablesSort::None {
      return Ok(tables_mut);
    }

    let mut locks = tables_mut.iter().fold(
      HashMap::<String, HashSet<String>>::new(),
      |mut acc, table| {
        if table.foreign_keys.len() == 0 {
          return acc;
        }

        match self {
          TablesSort::None => panic!("TablesSort::Never none"),
          TablesSort::ToDelete => {
            for fk in table.foreign_keys.iter() {
              match acc.get_mut::<String>(&fk.id()) {
                Some(x) => { x.insert(table.id()); },
                None => { acc.insert(fk.id(), HashSet::from([ table.id() ])); },
              }
            }
          },
          TablesSort::ToInsert => {
            let table_id = table.id();

            if !acc.contains_key(&table_id) { 
              acc.insert(table_id.clone(), HashSet::new());
            }

            let dependencies = acc.get_mut(&table_id).unwrap();
            for fk in table.foreign_keys.iter() {
              dependencies.insert(fk.id());
            }
          },
        }

        acc
      }
    );
    let by_dependency = locks.iter().fold(
      HashMap::<String, HashSet<String>>::new(),
      |mut acc, (table, deps)| {
        for dep in deps.iter() {
          let dep_targets;
          match acc.get_mut(dep) {
            Some(x) => { x.insert(table.clone()); },
            None => {
              dep_targets = HashSet::from([ table.to_string() ]);
              acc.insert(dep.clone(), dep_targets);
            }
          }
        }

        acc
      }
    );

    println!("tables={:?}", tables_mut);
    println!("locks={:?}", locks);
    println!("by_dependency={:?}", by_dependency);

    let mut result = vec![];
    let mut empty_wave;
    
    loop {
      empty_wave = true;
      let mut new_tables = vec![];
      for t in tables_mut.into_iter() {
        let id: String = t.id();
        if locks.get(&id).map(|x| x.len() > 0).unwrap_or(false) {
          new_tables.push(t);
          continue;
        }

        let t_id = t.id();
        let subject_temp = HashSet::new();
        let subject = by_dependency.get(&t_id).unwrap_or(&subject_temp);
        for l in subject.iter() {
          match locks.get_mut(l) {
            Some(subject_locks) => { subject_locks.remove(&t_id); },
            None => { },
          }
        }
        
        result.push(t);
        empty_wave = false;
      }

      if empty_wave && new_tables.len() > 0 {
        return Err(format!(
          "Tables.select [founded ring]: {}", 
          new_tables
            .iter()
            .map(|x| x.id())
            .collect::<Vec<String>>()
            .join(", ")
        ));
      }

      if new_tables.len() == 0 {
        break;
      }

      tables_mut = new_tables;
    }

    Ok(result)
  }
}