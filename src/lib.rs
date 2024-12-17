#![feature(portable_simd)]
#![feature(avx512_target_feature)]

mod util;

use std::sync::LazyLock;

use seq_macro::seq;

type AocFn = fn(&str) -> u64;

static IS_BENCH: LazyLock<bool> = LazyLock::new(|| {
    std::env::args().any(|arg| arg == "--bench")
});

seq! {
    N in 1..=17 {
        #(
            pub mod day~N;
        )*
        pub static FUNCS: &[(AocFn, AocFn)] = &[
            #(
                (day~N::part1 as _, day~N::part2 as _),
            )*
        ];
    }
}
