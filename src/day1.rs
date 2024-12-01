use std::hint::assert_unchecked;

use crate::util::FastParse;

// 12.1us
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

// 5.1us
pub fn part2(input: &str) -> u64 {
    let mut num_counts = vec![0u16; 99999];
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    let lines = input.len() / line_length;
    let mut appeared = Vec::with_capacity(lines);

    for_each_line(input, |num1, num2| {
        appeared.push(num1);
        num_counts[num2 as usize] += 1;
    });
    appeared
        .iter()
        .map(|&num| num * num_counts[num as usize] as u64)
        .sum()
}

fn for_each_line<F>(input: &str, mut f: F)
where
    F: FnMut(u64, u64),
{
    let mut input = input.as_bytes();
    let line_length = memchr::memchr(b'\n', input).unwrap();
    // Length of first column
    let first_col_len = memchr::memchr(b' ', &input[..line_length]).unwrap();
    // SAFETY: `memchr` returns a value less than the length
    unsafe { assert_unchecked(first_col_len < line_length) };

    // Offset from start to second column
    let second_col_offset = memchr::memrchr(b' ', &input[..line_length]).unwrap() + 1;
    // SAFETY: `memchr` returns a value less than the length
    unsafe { assert_unchecked(second_col_offset < line_length) };
    assert!(second_col_offset > first_col_len);

    let second_col_len = line_length - second_col_offset;

    while !input.is_empty() {
        assert!(input.len() > line_length);

        let num1 = u64::fast_parse_unchecked(&input[..first_col_len]);
        let num2 = u64::fast_parse_unchecked(&input[second_col_offset..][..second_col_len]);
        f(num1, num2);
        input = &input[line_length + 1..];
    }
}
