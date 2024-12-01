use rustc_hash::FxHashMap;

use crate::util::FastParse;

// 14.1us
pub fn part1(mut input: &str) -> u64 {
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    let second_col_offset = memchr::memrchr(b' ', input[..line_length].as_bytes()).unwrap() + 1;
    let lines = input.len() / line_length;
    let mut left = Vec::with_capacity(lines);
    let mut right = Vec::with_capacity(lines);
    while !input.is_empty() {
        let (num1, _) = u64::fast_parse(input).unwrap();
        input = &input[second_col_offset..];
        left.push(num1);
        let (num2, len) = u64::fast_parse(input).unwrap();
        input = &input[len + 1..];
        right.push(num2);
    }
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum()
}

// 10.1us
pub fn part2(mut input: &str) -> u64 {
    #[derive(Default)]
    struct Num {
        count: u64,
        appeared_left: bool,
    }
    let mut nums: FxHashMap<u64, Num> = FxHashMap::default();
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    let second_col_offset = memchr::memrchr(b' ', input[..line_length].as_bytes()).unwrap() + 1;
    let lines = input.len() / line_length;
    nums.reserve(lines);
    debug_assert!(input.ends_with('\n'));
    while !input.is_empty() {
        let (num1, _) = u64::fast_parse(input).unwrap();
        input = &input[second_col_offset..];
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
