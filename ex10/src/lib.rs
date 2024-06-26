extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemStruct, Stmt};

#[proc_macro_derive(JsonEncoder, attributes(json))]
pub fn json_encoder(input: TokenStream) -> TokenStream {
    let st_input: ItemStruct = parse_macro_input!(input as ItemStruct);
    let st_name = st_input.ident.clone();
    let mut json_stmt: Vec<Stmt> = vec![];

    for _field in st_input.fields {
        let mut displayname : String = Default::default(); 

        for attr in _field.attrs {
            if attr.path().is_ident("json") {
                // found
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("tag") {
                        displayname = meta.value()?.to_string();
                        return Ok(());
                    }

                    Err(meta.error("unsupported attribute"))

                });
            }
        }

        if let Some(_ident) = &_field.ident {
            let st: Stmt = parse_quote!(
                jstr.push_str(format!( concat!(#displayname," : {} "), self.#_ident).as_str());
            );

            json_stmt.push(st);
        }
    }

    let tks = quote! {
        impl JsonConverter for #st_name {
            fn json_get(&self) -> String {
                let mut jstr = String::new();
                jstr.push_str("{");
                #(#json_stmt)*
                jstr.push_str("}");
                jstr
            }
        }
    };

    TokenStream::from(tks)
}

#[proc_macro]
pub fn create_json_trait(_: TokenStream) -> TokenStream {
    let tks = quote! {
        pub trait JsonConverter {
            fn json_get(&self) -> String;
        }
    };

    TokenStream::from(tks)
}
