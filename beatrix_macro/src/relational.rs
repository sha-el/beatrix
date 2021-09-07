use proc_macro::TokenStream;
use quote::quote;

use syn::{Lit, Meta, MetaList, MetaNameValue, NestedMeta};

pub fn impl_relational_model(ast: &syn::DeriveInput) -> TokenStream {
    println!("{:#?}", ast);

    let gen = quote! {};
    gen.into()
}
