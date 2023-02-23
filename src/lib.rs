extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;

use digest::Digest;
use quote::quote;
use syn::{LitInt, parse_macro_input};
use syn::parse::{Parse, ParseStream};

fn trim_to_fr32(buff: &mut [u8; 32]) {
    // strip last two bits, to ensure result is in Fr.
    buff[31] &= 0b0011_1111;
}

fn gen(layer: usize) -> Vec<[u8; 32]> {
    let mut out = Vec::with_capacity(layer);
    let mut hash = [0u8; 32];

    for _ in 0..layer {
        let mut input = [0u8; 64];
        input[..32].copy_from_slice(&hash);
        input[32..].copy_from_slice(&hash);
        hash.copy_from_slice(sha2::Sha256::digest(input).as_slice());
        trim_to_fr32(&mut hash);
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
