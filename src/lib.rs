extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(BaseModel)]
pub fn base_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl BaseModel for #name {

            fn get_table_name(&self) -> &str {
                return TABLE_NAME;
            }

            fn get_alias(&self) -> &str {
                return ALIAS;
            }

            fn get_fields_count(&self) -> u16{
                return FIELDS.len().try_into().unwrap();
            }

            fn get_field_sql(&self, alias: &str) -> String {
                return get_field_sql(alias);
            }

            fn get_field_sql_not_pk(&self, alias: &str) -> String {
                let mut columns = String::from("");
                for c in &FIELDS[1..] {
                    if "" != columns {
                        columns.push_str(", ");
                    }
                    if "" != alias {
                        columns.push_str(&format!("{}.{}" , alias, c));
                    } else {
                        columns.push_str(&format!("{}" , c));
                    }
                }
                return columns;
            }

            fn get_params_insert(&self) -> (r2d2_mysql::mysql::Params, String, String) {
                let mut columns = String::from("");
                let mut keys = String::from("");

                for c in &FIELDS[1..] {
                    if "" != columns {
                        columns.push_str(", ");
                        keys.push_str(", ");
                    }
                    columns.push_str(&format!("{}" , c));
                    keys.push_str(&format!(":{}" , c));
                }

                return (params! {
                    "created_at" => self.created_at,
                    "updated_at" => self.updated_at,
                    "user_name" => self.user_name.to_string(),
                    "state" => self.state,
                }, columns, keys);
            }

            fn get_params_update_by_pk(&self) -> (Params, String, String) {
                let mut columns = String::from("");

                for c in &FIELDS[2..] {
                    if "" != columns {
                        columns.push_str(", ");
                    }
                    columns.push_str(&format!("{}=:{}" , c, c));
                }

                return (params! {
                    "updated_at" => self.updated_at,
                    "user_name" => self.user_name.to_string(),
                    "state" => self.state,
                    "id" => self.id,
                }, columns, String::from(format!("{}=:{}",  FIELDS[0], FIELDS[0])))
            }

            fn set_pk(&mut self, pk: u64) {
                self.id = pk;
            }

            fn set_created_at(&mut self, at: u64) {
                self.created_at = at;
            }

            fn set_updated_at(&mut self, at: u64) {
                self.updated_at = at;
            }

        }
    };
    TokenStream::from(expanded)
}

// #[derive(BaseModel)]
// use crate::BaseModel;
// use crate::model::BaseModel;
// impl model::BaseModel for TestUser {
//
//     fn get_table_name(&self) -> &str {
//         return TABLE_NAME;
//     }
//
//     fn get_alias(&self) -> &str {
//         return ALIAS;
//     }
//
//     fn get_fields_count(&self) -> u16{
//         return FIELDS.len().try_into().unwrap();
//     }
//
//     fn get_field_sql(&self, alias: &str) -> String {
//         return get_field_sql(alias);
//     }
//
//     fn get_field_sql_not_pk(&self, alias: &str) -> String {
//         let mut columns = String::from("");
//         for c in &FIELDS[1..] {
//             if "" != columns {
//                 columns.push_str(", ");
//             }
//             if "" != alias {
//                 columns.push_str(&format!("{}.{}" , alias, c));
//             } else {
//                 columns.push_str(&format!("{}" , c));
//             }
//         }
//         return columns;
//     }
//
//     fn get_params_insert(&self) -> (r2d2_mysql::mysql::Params, String, String) {
//         let mut columns = String::from("");
//         let mut keys = String::from("");
//
//         for c in &FIELDS[1..] {
//             if "" != columns {
//                 columns.push_str(", ");
//                 keys.push_str(", ");
//             }
//             columns.push_str(&format!("{}" , c));
//             keys.push_str(&format!(":{}" , c));
//         }
//
//         return (params! {
//             "created_at" => self.created_at,
//             "updated_at" => self.updated_at,
//             "user_name" => self.user_name.to_string(),
//             "state" => self.state,
//         }, columns, keys);
//     }
//
//     fn get_params_update_by_pk(&self) -> (Params, String, String) {
//         let mut columns = String::from("");
//
//         for c in &FIELDS[2..] {
//             if "" != columns {
//                 columns.push_str(", ");
//             }
//             columns.push_str(&format!("{}=:{}" , c, c));
//         }
//
//         return (params! {
//             "updated_at" => self.updated_at,
//             "user_name" => self.user_name.to_string(),
//             "state" => self.state,
//             "id" => self.id,
//         }, columns, String::from(format!("{}=:{}",  FIELDS[0], FIELDS[0])))
//     }
//
//     fn set_pk(&mut self, pk: u64) {
//         self.id = pk;
//     }
//
//     fn set_created_at(&mut self, at: u64) {
//         self.created_at = at;
//     }
//
//     fn set_updated_at(&mut self, at: u64) {
//         self.updated_at = at;
//     }
//
// }
