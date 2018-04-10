#![feature(proc_macro)]

extern crate sorting;
use sorting::static_sort;

#[test]
fn bose_nelson() {
    let mut x: [u64; 3] = [2, 0, 1];
    static_sort! { bose_nelson, 3, x };
    println!("{:?}", x);
}
