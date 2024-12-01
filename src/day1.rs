use rustc_hash::FxHashMap;

use crate::util::FastParse;

// 12.9us
pub fn part1(input: &str) -> u64 {
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    let lines = input.len() / line_length;
    let mut left = Vec::with_capacity(lines);
    let mut right = Vec::with_capacity(lines);
    for_each_line(input, |num1, num2| {
        left.push(num1);
        right.push(num2);
    });

    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum()
}

// 9.6us
pub fn part2(input: &str) -> u64 {
    #[derive(Default)]
    struct Num {
        count: u64,
        appeared_left: bool,
    }
    let mut nums: FxHashMap<u64, Num> = FxHashMap::default();
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();

    let lines = input.len() / line_length;
    nums.reserve(lines);
    for_each_line(input, |num1, num2| {
        nums.entry(num1).or_default().appeared_left = true;
        nums.entry(num2).or_default().count += 1;
    });
    nums.iter()
        .filter(|(_, num)| num.appeared_left)
        .map(|(val, num)| val * num.count)
        .sum()
}

fn for_each_line<F>(mut input: &str, mut f: F)
where
    F: FnMut(u64, u64),
{
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    // Length of first column
    let first_col_len = memchr::memchr(b' ', input[..line_length].as_bytes()).unwrap();
    // Offset from start to second column
    let second_col_offset = memchr::memrchr(b' ', input[..line_length].as_bytes()).unwrap() + 1;
    let second_col_len = line_length - second_col_offset;
    while !input.is_empty() {
        let num1 = u64::fast_parse_unchecked(&input[..first_col_len]);
        let num2 = u64::fast_parse_unchecked(&input[second_col_offset..][..second_col_len]);
        f(num1, num2);
        input = &input[line_length + 1..];
    }
}
