use rustc_hash::FxHashMap;

use crate::util::FastParse;

// 26.8us
pub fn part1(input: &str) -> u64 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(char::is_whitespace).unwrap();
            (
                u64::fast_parse(l).unwrap().0,
                u64::fast_parse(r.trim_ascii_start()).unwrap().0,
            )
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum()
}

// 10.8us
pub fn part2(mut input: &str) -> u64 {
    #[derive(Default)]
    struct Num {
        count: u64,
        appeared_left: bool,
    }
    let mut nums: FxHashMap<u64, Num> = FxHashMap::default();
    // Divide total length by length of first line to preallocate
    nums.reserve(input.len() / memchr::memchr(b'\n', input.as_bytes()).unwrap());
    debug_assert!(input.ends_with('\n'));
    while !input.is_empty() {
        let (num1, len) = u64::fast_parse(input).unwrap();
        input = &input[len..].trim_ascii_start();
        nums.entry(num1).or_default().appeared_left = true;
        let (num2, len) = u64::fast_parse(input).unwrap();
        input = &input[len + 1..];
        nums.entry(num2).or_default().count += 1;
    }
    nums.iter()
        .filter(|(_, num)| num.appeared_left)
        .map(|(val, num)| val * num.count)
        .sum()
}
