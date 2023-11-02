mod field;

pub(crate) use field::*;

use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(FromRow, attributes(from_row, from_json, from_nullable_json))]
pub fn from_row_derive(input: TokenStream) -> TokenStream {
  let parsed = parse_macro_input!(input as DeriveInput);
  let struct_ident = parsed.ident;

  let struct_meta = match parsed.data {
    syn::Data::Struct(x) => x,
    syn::Data::Enum(_) => panic!("FromRow [{}]: enums not supported", struct_ident),
    syn::Data::Union(_) => panic!("FromRow [{}]: unions not supported", struct_ident),
  };

  let mut fields = vec![];
  for field_meta in struct_meta.fields.iter() {
    fields.push(
      match Field::parse(field_meta) {
        Ok(x) => {
          let ident = x.ident;

          match x.from_row {
            Some(x) => quote!(#ident: #x,),
            None => {
              let column_name = x.column_name;
              let ty = x.ty;

              match x.default {
                Some(d) => quote!(#ident: { 
                  match row.try_get(#column_name) {
                    Ok(x) => x,
                    Err(e) => {
                      let message = format!("{:?}", e);
                      if "a Postgres value was `NULL`" != &e.into_source().map_or("".into(), |x| format!("{}", x)) {
                        panic!("{}", message);
                      }
                      
                      #d
                    }
                  }
                },),
                None => quote!(#ident: row.get::<&str, #ty>(#column_name),)
              }
            }
          }
        },
        Err(e) => panic!("FromRow [{}]\n\t{}", struct_ident, e)
      }
    );
  }

  TokenStream::from(
    quote! {
      impl From<&postgres::Row> for #struct_ident {
        fn from(row: &postgres::Row) -> Self { 
          Self {
            #(#fields)*
          }
        }
      }

      impl From<postgres::Row> for #struct_ident {
        fn from(row: postgres::Row) -> Self { 
          Self {
            #(#fields)*
          }
        }
      }
    }
  )
}

#[proc_macro_derive(FromRows)]
pub fn from_rows_derive(input: TokenStream) -> TokenStream {
  let struct_ident = parse_macro_input!(input as DeriveInput).ident;

  TokenStream::from(
    quote! {
      impl #struct_ident {
        pub fn from_rows(rows: Vec<postgres::Row>) -> Vec<Self> {
          rows.iter().map(|x| x.into()).collect::<Vec<#struct_ident>>() 
        }
      }
    }
  )
}