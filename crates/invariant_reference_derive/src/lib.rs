use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Invariant, attributes(invariant))]
pub fn derive_invariant(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let input = InvariantDeriveInput::from_derive_input(&input).unwrap();
    let struct_name = input.ident.clone();
    let num_proofs = input.num_proofs.unwrap_or(1);
    let bounds: proc_macro2::TokenStream = (0..num_proofs)
        .map(|i| {
            quote! {
                Self: invariant_reference::InvariantProof<#i>,
            }
        })
        .collect();
    let message = input.message.unwrap_or_else(|| struct_name.to_string());
    quote! {
        impl invariant_reference::Invariant for #struct_name
        where #bounds {
            const MESSAGE: &str = #message;
        }
    }
    .into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(invariant))]
struct InvariantDeriveInput {
    ident: syn::Ident,
    num_proofs: Option<usize>,
    message: Option<String>,
}
