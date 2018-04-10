#![feature(proc_macro, test)]

extern crate test;
extern crate rand;
extern crate static_sort;

use test::{black_box, Bencher};
use rand::{Rng, thread_rng};
use static_sort::{ssort_bose_nelson, ssort_batcher, ssort_bubble};


macro_rules! mk_bench {
    ($name:ident, $N:expr, $x:ident, $sort:stmt) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut arr = [0; $N];

            let mut rng = thread_rng();
            for i in 0..$N {
                arr[i] = rng.gen();
            }

            b.iter(|| {
                  let mut $x = arr;
                  for _ in 0..10 { $x = arr; $sort; }
                  $x[black_box(0)]
            })
        }
    }
}


macro_rules! bench_n {
    ($N:expr, $std_sort:ident, $ssort_bn:ident, $ssort_batcher:ident, $ssort_bubble:ident) => {
        mk_bench! { $std_sort, $N, x, x.sort_unstable() }
        mk_bench! { $ssort_bn, $N, x, ssort_bose_nelson!($N, x) }
        mk_bench! { $ssort_batcher, $N, x, ssort_batcher!($N, x) }
        mk_bench! { $ssort_bubble, $N, x, ssort_bubble!($N, x) }
    }
}


bench_n! { 3, std_3, ssort_bn_3, ssort_batcher_3, ssort_bubble_3 }
bench_n! { 4, std_4, ssort_bn_4, ssort_batcher_4, ssort_bubble_4 }
bench_n! { 5, std_5, ssort_bn_5, ssort_batcher_5, ssort_bubble_5 }
bench_n! { 6, std_6, ssort_bn_6, ssort_batcher_6, ssort_bubble_6 }
bench_n! { 7, std_7, ssort_bn_7, ssort_batcher_7, ssort_bubble_7 }
bench_n! { 8, std_8, ssort_bn_8, ssort_batcher_8, ssort_bubble_8 }
bench_n! { 9, std_9, ssort_bn_9, ssort_batcher_9, ssort_bubble_9 }
bench_n! { 10, std_10, ssort_bn_10, ssort_batcher_10, ssort_bubble_10 }
bench_n! { 11, std_11, ssort_bn_11, ssort_batcher_11, ssort_bubble_11 }
bench_n! { 12, std_12, ssort_bn_12, ssort_batcher_12, ssort_bubble_12 }
bench_n! { 13, std_13, ssort_bn_13, ssort_batcher_13, ssort_bubble_13 }
bench_n! { 14, std_14, ssort_bn_14, ssort_batcher_14, ssort_bubble_14 }
bench_n! { 15, std_15, ssort_bn_15, ssort_batcher_15, ssort_bubble_15 }
bench_n! { 16, std_16, ssort_bn_16, ssort_batcher_16, ssort_bubble_16 }
bench_n! { 17, std_17, ssort_bn_17, ssort_batcher_17, ssort_bubble_17 }
bench_n! { 18, std_18, ssort_bn_18, ssort_batcher_18, ssort_bubble_18 }
bench_n! { 19, std_19, ssort_bn_19, ssort_batcher_19, ssort_bubble_19 }
bench_n! { 20, std_20, ssort_bn_20, ssort_batcher_20, ssort_bubble_20 }
