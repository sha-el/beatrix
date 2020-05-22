extern crate proc_macro;

mod model;
mod meta;
mod error;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, NestedMeta, Meta};

#[proc_macro_derive(Entity, attributes(table_name))]
pub fn entity_macro_derive(input: TokenStream) -> TokenStream {
    let i2 = input.clone();
    for arg in parse_macro_input!(input as syn::AttributeArgs) {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                println!("{:?}", nv.lit.tokens);
                if nv.path.is_ident("table_name") {
                    if let syn::Lit::Str(lit) = nv.lit {
                        println!("*********{:?}", lit.value());
                    }
                }
                // if let syn::Lit::Str(lit) = nv.lit {
                //     println!("{:?}", lit.value());
                // }
            }
            _ => {}
        }
    };

    let ast = syn::parse(i2).unwrap();

    impl_entity(&ast)
}

fn impl_entity(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Entity for #name {
            fn table_name(&self) -> String {
                format!("{}", stringify!(#name)).to_lowercase()
            }
        }
    };
    gen.into()
}

// #[proc_macro_attribute]
// pub fn table_name(metadata: TokenStream, input: TokenStream) -> TokenStream {
//     print!("******{:?}", metadata);
//     print!("******{:?}", input);

//     let gen = quote! {
//         println!("hello")
//     };

//     gen.into()

// }