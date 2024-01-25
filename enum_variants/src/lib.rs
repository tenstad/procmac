extern crate proc_macro;

use proc_macro::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::{DeriveInput, Ident};

#[proc_macro]
pub fn enum_variant(stream: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = stream.into_iter().collect();
    let name = Ident::new(tokens[1].to_string().as_str(), Span::call_site().into());

    quote! {
        enum #name{

        };
    }
    .into()
}

#[proc_macro_derive(EnumVairant, attributes(value))]
pub fn enum_variant_derive(stream: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(stream).unwrap();
    let name = input.ident;

    quote! {
        impl #name {
            fn get_variant(&self) -> Foo {
                match self {

                }
            }
        };
    }
    .into()
}
