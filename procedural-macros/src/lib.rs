use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Describe only supports structs with named fields"),
        },
        _ => panic!("Describe only supports structs"),
    };

    let field_descriptions = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            format!("{}: {:?}", stringify!(#field_name), &self.#field_name)
        }
    });

    let expanded = quote! {
        impl Describe for #name {
            fn describe(&self) -> String {
                let fields = vec![#(#field_descriptions),*];

                format!("{} {{ {} }}", stringify!(#name), fields.join(", "))
            }
        }
    };

    expanded.into()
}
