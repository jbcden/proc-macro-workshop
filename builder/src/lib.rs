extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let buildee_ident = &ast.ident;

    let expanded = quote! {
        impl #buildee_ident {
            pub fn builder() {}
        }
    };


    expanded.into()
}
