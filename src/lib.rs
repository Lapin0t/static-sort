#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use] extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Expr, LitInt, Block, IntSuffix};
use syn::synom::Synom;


mod bose_nelson;
mod batcher;
mod bubble;


type Net = Vec<(u64, u64)>;


fn comparator(x: Expr, i: u64, j: u64) -> Block {
    let i = LitInt::new(i, IntSuffix::None, Span::call_site());
    let j = LitInt::new(j, IntSuffix::None, Span::call_site());
    parse_quote! {
       { 
            let c = #x[#i] < #x[#j];
            let m = if c { #x[#i] } else { #x[#j] };
            #x[#j] = if c { #x[#j] } else { #x[#i] };
            #x[#i] = m;
       }
    }
}


struct SortArgs {
    len: LitInt,
    array: Expr,
}


impl Synom for SortArgs {
    named!(parse -> Self, do_parse!(
        len: syn!(LitInt) >>
        punct!(,) >>
        array: syn!(Expr) >>
        (SortArgs { array, len })
    ));
}


macro_rules! mk_ssort {
    ($name:ident, $gen:path) => {
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            let SortArgs { len, array } = syn::parse(input).unwrap();

            let swaps = $gen(len.value()).into_iter()
                .map(|(i, j)| comparator(array.clone(), i, j));

            let out = quote! { #(#swaps)* };
            out.into()
        }
    }
}

mk_ssort! { ssort_bose_nelson, bose_nelson::bose_nelson }
mk_ssort! { ssort_batcher, batcher::batcher }
mk_ssort! { ssort_bubble, bubble::bubble }
