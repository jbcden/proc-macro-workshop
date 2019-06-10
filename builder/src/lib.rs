extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Data, Fields, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let buildee_ident = &ast.ident;
    let builder_name = format!("{}Builder", buildee_ident);
    let builder_ident = Ident::new(&builder_name, ast.ident.span());

    let struct_definition = generate_builder_struct_definition(&builder_ident, &ast.data);
    let default_builder_constructor = generate_default_builder_constructor(&builder_ident, &ast.data);
    let setter_functions = generate_builder_setters(&builder_ident, &ast.data);

    // eprintln!("TOKENS: {:#?}", struct_definition);
    let expanded = quote! {
        #struct_definition

      impl #buildee_ident {
        pub fn builder() -> #builder_ident {
          #default_builder_constructor
        }
      }

      impl #builder_ident {
        #setter_functions
      }
    };

    proc_macro::TokenStream::from(expanded)
}

/// These functions are based on https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs#L47
fn generate_builder_struct_definition(ident: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let fields = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;

                        return quote! {
                          #name: Option<#ty>,
                        }
                    });

                    let tokens = quote! {
                        pub struct #ident {
                          #(#fields)*
                        }
                    };
                    tokens.into()
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

fn generate_default_builder_constructor(ident: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let fields = fields.named.iter().map(|f| {
                        let name = &f.ident;

                        return quote! {
                          #name: None,
                        }
                    });

                    let tokens = quote! {
                        #ident {
                          #(#fields)*
                        }
                    };
                    tokens.into()
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

fn generate_builder_setters(ident: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let fields = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;

                        return quote! {
                          pub fn #name(&mut self, #name: #ty) -> &mut Self {
                            self.#name = Some(#name);
                            self
                          }
                        }
                    });

                    let tokens = quote! {
                      #(#fields)*
                    };
                    tokens.into()
                },
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}
