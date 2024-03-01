extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn impl_hello_world(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let gen = impl_hello_world(&ast);

    gen
}
