extern crate proc_macro;
use proc_macro::TokenStream;

mod mongo;

#[proc_macro_derive(MongoModel, attributes(entity))]
pub fn derive_mongo_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    mongo::impl_mongo(&ast)
}


