#![feature(portable_simd)]
#![feature(avx512_target_feature)]

mod util;

use seq_macro::seq;

type AocFn = fn(&str) -> u64;

seq! {
    N in 1..=6 {
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
