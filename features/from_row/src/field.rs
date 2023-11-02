use syn::{
  punctuated::Punctuated,
  Expr, Ident, MetaNameValue, Token, Type, parse_quote, Attribute
};

pub struct Field {
  pub ident: Ident,
  pub ty: Type,
  pub column_name: Expr,
  pub from_row: Option<Expr>,
  pub default: Option<Expr>,
}

#[derive(PartialEq)]
enum AttributeType {
  FromRow,
  FromJson,
  FromNullableJson,
}

impl Field {
  pub fn parse(field: &syn::Field) -> Result<Self, String> {
    let ident = match field.ident.as_ref() {
      Some(x) => x,
      None => return Err("field without ident".into())
    };
    let ident_str = ident.to_string();
    let mut column_name: Expr = parse_quote!(#ident_str);
    let mut from_row = None;
    let mut default = None;

    for attr in field.attrs.iter() {
      let attr_type = match AttributeType::from(attr) {
        Some(ty) => ty,
        None => { continue }
      };

      let parsed: Punctuated<MetaNameValue, Token![,]> = attr
        .parse_args_with(Punctuated::parse_terminated)
        .map_err(|e| format!(r#"{}Parse field "{}" error: {:?}"#, ERROR_PREFIX, ident, e))?;

      let mut has_from = false;
      let mut has_other = false;
      for name_value in parsed.into_iter() {
        if name_value.path.is_ident("from") {
          if attr_type != AttributeType::FromRow {
            return Err(format!(
              r#"{}Parse field "{}" error: "from" can be used only in from_row attribute"#, 
              ERROR_PREFIX, 
              ident
            ));
          }

          has_from = true;
          from_row = Some(name_value.value);
          continue;
        }

        has_other = true;
        
        if name_value.path.is_ident("column") {
          column_name = name_value.value;
          continue;
        }

        if name_value.path.is_ident("default") {
          if attr_type == AttributeType::FromNullableJson {
            return Err(format!(
              r#"{}Parse field "{}" error: "default" can't be used in from_nullable_json"#, 
              ERROR_PREFIX, 
              ident
            ));
          }

          default = Some(name_value.value);
          continue;
        }

        return Err(format!(
          r#"{}Parse field "{}" error: unhandled attribute"#, 
          ERROR_PREFIX, 
          ident,
        ))
      }

      match attr_type {
        AttributeType::FromRow => { 
          if has_from && has_other {
            return Err(format!(
              r#"{}Parse field "{}" error: "from" can't be used with other settings"#, 
              ERROR_PREFIX, 
              ident
            ));
          }
        },
        AttributeType::FromJson => { 
          from_row = Some(
            match default.clone() {
              Some(def) => parse_quote!({
                let v: Option<serde_json::Value> = row.get(#column_name);
                v.map(|x| serde_json::from_value(x).unwrap()).unwrap_or(#def)
              }),
              None => parse_quote!(serde_json::from_value(row.get(#column_name)).unwrap())
            }
          );
        },
        AttributeType::FromNullableJson => {
          from_row = Some(parse_quote!({
            let v: Option<serde_json::Value> = row.get(#column_name);
            v.map(|x| serde_json::from_value(x).unwrap())
          }));
        },
      }
    }

    Ok(
      Self {
        ident: ident.clone(),
        ty: field.ty.clone(),
        column_name,
        from_row: from_row.clone(),
        default: default.clone()
      }
    )
  }
}

impl AttributeType {
  pub fn from(attr: &Attribute) -> Option<Self> {
    if attr.path().is_ident("from_row") {
      return Some(Self::FromRow);
    }

    if attr.path().is_ident("from_json") {
      return Some(Self::FromJson);
    }

    if attr.path().is_ident("from_nullable_json") {
      return Some(Self::FromNullableJson);
    }

    return None;
  }
}

const ERROR_PREFIX: &str = "  ";