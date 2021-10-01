use beatrix_core::relational::field::FieldDetails;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Fields, punctuated::Pair};

pub fn generate_fields(token: &str) -> Vec<FieldDetails> {
    let ast: TokenStream = token.parse().expect("Invalid syntax");
    let ast: DeriveInput = syn::parse2(ast).expect("Invalid syntax");

    if let Data::Struct(data) = ast.data {
        if let Fields::Named(field) = &data.fields {
            field
                .named
                .clone()
                .into_pairs()
                .into_iter()
                .map(|paired| {
                    let field = match paired {
                        Pair::Punctuated(t, _) => t,
                        Pair::End(t) => t,
                    };
                    FieldDetails { field, alias: None }
                })
                .collect::<_>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}
