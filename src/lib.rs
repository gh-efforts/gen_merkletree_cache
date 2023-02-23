extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;

use digest::Digest;
use quote::quote;
use syn::{LitInt, parse_macro_input};
use syn::parse::{Parse, ParseStream};

fn gen(layer: usize) -> Vec<[u8; 32]> {
    let mut out = Vec::with_capacity(layer);

    let hash_ref = sha2::Sha256::digest([0u8; 64]);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(hash_ref.as_slice());
    out.push(hash);

    for _ in 1..layer {
        let mut input = [0u8; 64];
        input[..32].copy_from_slice(&hash);
        input[32..].copy_from_slice(&hash);
        hash.copy_from_slice(sha2::Sha256::digest(input).as_slice());

        out.push(hash);
    };
    out
}

struct Layer {
    value: usize,
}

impl Parse for Layer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: LitInt = input.parse()?;
        let value = lit.base10_parse::<usize>()?;
        Ok(Layer { value })
    }
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let layer = parse_macro_input!(input as Layer);
    let cache = gen(layer.value);

    let cache: Vec<proc_macro2::TokenStream> = cache.into_iter()
        .map(|arr| quote!([#(#arr),*]))
        .collect();

    let tokens = quote! {
        [#(#cache),*]
    };
    tokens.into()
}
