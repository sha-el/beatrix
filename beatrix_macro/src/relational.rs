use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Pair, Data, DeriveInput, Fields};

pub fn custom_table_name(ast: &syn::DeriveInput) -> String {
    let struct_name = &ast.ident;
    let mut table_name = struct_name.to_string();

    for attr in ast.attrs.clone().into_iter() {
        if attr.path.segments[0].ident == "table_name" {
            let value = attr.tokens.to_string();
            let value = value.split_once("=").unwrap();
            table_name = value.1.trim().replacen("\"", "", 2);
        }
    }

    table_name
}

pub fn gen_fields(ast: &DeriveInput) -> TokenStream {
    let ast_string = quote! { #ast }.to_string();
    let fields = if let Data::Struct(data) = &ast.data {
        if let Fields::Named(field) = &data.fields {
            field
                .named
                .clone()
                .into_pairs()
                .enumerate()
                .map(|(index, paired)| {
                    let field = match paired {
                        Pair::Punctuated(t, _) => t,
                        Pair::End(t) => t,
                    };
                    let field_name: TokenStream = field
                        .ident
                        .unwrap()
                        .to_string()
                        .parse()
                        .expect("Invalid field name");
                    let field_type = field.ty;
                    quote! {
                        pub fn #field_name() -> (#field_type, beatrix::relational::field::FieldDetails) {
                            (Default::default(), beatrix::helpers::generate_fields(#ast_string).remove(#index))
                        }
                    }
                })
                .collect::<_>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let mut gen = quote! {};
    gen.extend(fields);
    gen
}

pub fn impl_relational_model(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let table_name = custom_table_name(ast);
    let fields = gen_fields(ast);
    let ast = quote! { #ast }.to_string();

    quote! {
        impl beatrix::relational::table::Table for #struct_name {
            fn table_details() -> beatrix::relational::table::TableDetails {
                beatrix::relational::table::TableDetails {
                    name: (#table_name).to_string(),
                    alias: None,
                }
            }

            fn fields() -> Vec<beatrix::relational::field::FieldDetails> {
                beatrix::helpers::generate_fields(#ast)
            }

            fn select() -> beatrix::relational::select::Select {
                beatrix::relational::select::Select::new(Self::fields(), vec![Self::table_details()])
            }
        }

        impl #struct_name {
            #fields
        }
    }
}
