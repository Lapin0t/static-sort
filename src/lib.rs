#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use] extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Ident, LitInt, Block, IntSuffix};
use syn::synom::Synom;


enum SortAlg {
    BoseNelson,
    Batcher,
    Hibbard,
}


struct SortArgs {
    algname: Ident,
    name: Ident,
    len: LitInt,
}


impl Synom for SortArgs {
    named!(parse -> Self, do_parse!(
        algname: syn!(Ident) >>
        punct!(,) >>
        len: syn!(LitInt) >>
        punct!(,) >>
        name: syn!(Ident) >>
        (SortArgs { algname, name, len })
    ));
}


fn gen_swaps(_alg: SortAlg, _len: u64) -> Vec<(u64, u64)> {
    vec![(0, 1)]
}


fn do_swap(a: Ident, i: u64, j: u64) -> Block {
    let i = LitInt::new(i, IntSuffix::None, Span::call_site());
    let j = LitInt::new(j, IntSuffix::None, Span::call_site());
    parse_quote! {
       { 
            let c = #a[#i] < #a[#j];
            let m = if c { #a[#i] } else { #a[#j] };
            #a[#j] = if c { #a[#j] } else { #a[#i] };
            #a[#i] = m;
       }
    }
}


#[proc_macro]
pub fn static_sort(input: TokenStream) -> TokenStream {
    let SortArgs { algname, name, len } = syn::parse(input).unwrap();

    let alg = match algname.as_ref() {
        "bose_nelson" => SortAlg::BoseNelson,
        "batcher" => SortAlg::Batcher,
        "hibbard" => SortAlg::Hibbard,
        _ => {
            algname.span().unstable().error("unknown algorithm").emit();
            return TokenStream::empty();
        }
    };

    let swaps = gen_swaps(alg, len.value())
        .into_iter()
        .map(|(i, j)| do_swap(name, i, j));

    let out = quote! { #(#swaps);* };

    out.into()
}
