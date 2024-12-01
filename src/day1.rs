use std::{
    hint::assert_unchecked,
    simd::{LaneCount, Simd, SupportedLaneCount, num::SimdUint},
};

// 11.2us
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

// 4.1us
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

    while !input.is_empty() {
        assert!(input.len() > line_length);

        let (num1, num2) = parse_line_simd(input, first_col_len, second_col_offset, line_length);
        f(num1, num2);
        input = &input[line_length + 1..];
    }
}

fn parse_line_simd(
    input: &[u8],
    first_col_len: usize,
    second_col_offset: usize,
    line_length: usize,
) -> (u64, u64) {
    assert!(input.len() >= second_col_offset);
    assert!(input.len() >= first_col_len);
    assert!(input.len() >= line_length);

    match (first_col_len, second_col_offset, line_length) {
        (5, 8, 13) => (
            simd_parse_start::<8, 5>(input[..second_col_offset].try_into().unwrap()),
            simd_parse_end::<8, 3>(input[first_col_len..line_length].try_into().unwrap()),
        ),
        (1, 4, 5) => (
            simd_parse_start::<4, 1>(input[..second_col_offset].try_into().unwrap()),
            simd_parse_end::<4, 3>(input[first_col_len..line_length].try_into().unwrap()),
        ),
        _ => unimplemented!(),
    }
}

fn simd_parse_start<const INP_LEN: usize, const NUM_SIZE: usize>(line: &[u8; INP_LEN]) -> u64
where
    LaneCount<INP_LEN>: SupportedLaneCount,
{
    let multipliers = Simd::from(std::array::from_fn(|i| 10u64.pow(i as u32))).reverse();
    let mask = Simd::from(std::array::from_fn(
        |i| if i < NUM_SIZE { u64::MAX } else { 0 },
    ));
    let line = Simd::<u8, INP_LEN>::load_or_default(line);
    let digits = line - Simd::splat(b'0');
    let digits: Simd<u64, INP_LEN> = digits.cast();
    let digits = digits & mask;
    (digits * multipliers).reduce_sum() / 10u64.pow((INP_LEN - NUM_SIZE) as u32)
}

fn simd_parse_end<const INP_LEN: usize, const GAP_LEN: usize>(line: &[u8; INP_LEN]) -> u64
where
    LaneCount<INP_LEN>: SupportedLaneCount,
{
    let multipliers = Simd::from(std::array::from_fn(|i| 10u64.pow(i as u32))).reverse();
    let mask = Simd::from(std::array::from_fn(
        |i| if i >= GAP_LEN { u64::MAX } else { 0 },
    ));
    let line = Simd::<u8, INP_LEN>::load_or_default(line);
    let digits = line - Simd::splat(b'0');
    let digits: Simd<u64, INP_LEN> = digits.cast();
    let digits = digits & mask;
    (digits * multipliers).reduce_sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simd() {
        assert_eq!(parse_line_simd(b"01234   56789", 5, 8, 13), (1234, 56789));
    }
}
