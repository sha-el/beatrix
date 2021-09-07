extern crate proc_macro;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod mongo;
mod relational;

#[proc_macro_derive(MongoModel, attributes(entity))]
pub fn derive_mongo_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    mongo::impl_mongo(&ast)
}

#[proc_macro_derive(RelationalModel, attributes(table_name))]
pub fn derive_relational_model(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input.clone()).unwrap();
    relational::impl_relational_model(&ast)
}
