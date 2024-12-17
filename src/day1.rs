use std::{
    hint::assert_unchecked,
    simd::{LaneCount, Simd, SupportedLaneCount, num::SimdUint},
};

use either::Either;

// 12.8us
pub fn part1(input: &str) -> either::Either<u64, String> {
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
    Either::Left(left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum())
}

// 5.4us
pub fn part2(input: &str) -> either::Either<u64, String> {
    let mut num_counts = vec![0u8; 99999];
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    let lines = input.len() / line_length;
    let mut appeared = Vec::with_capacity(lines);

    for_each_line(input, |num1, num2| {
        appeared.push(num1);
        num_counts[num2 as usize] += 1;
    });
    Either::Left(
        appeared
            .iter()
            .map(|&num| num * num_counts[num as usize] as u64)
            .sum(),
    )
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

        let (num1, num2) = parse_line_simd(&input[..line_length], first_col_len, second_col_offset);
        f(num1, num2);
        input = &input[line_length + 1..];
    }
}

fn parse_line_simd(line: &[u8], first_col_len: usize, second_col_offset: usize) -> (u64, u64) {
    assert!(line.len() >= second_col_offset);
    assert!(line.len() >= first_col_len);

    // SAFETY: Buy a better cpu :ferrisclueless:
    match (first_col_len, second_col_offset) {
        (5, 8) => unsafe { simd_parse_inner::<13, 5, 16>(line.try_into().unwrap()) },
        (1, 4) => unsafe { simd_parse_inner::<5, 1, 8>(line.try_into().unwrap()) },
        _ => unimplemented!(),
    }
}

const fn get_multipliers<const LANE_SIZE: usize, const INP_LEN: usize, const NUM_SIZE: usize>()
-> Simd<u32, LANE_SIZE>
where
    LaneCount<LANE_SIZE>: SupportedLaneCount,
{
    let mut arr = [0; LANE_SIZE];
    let mut i = 0;
    while i < LANE_SIZE {
        if i < NUM_SIZE {
            arr[i] = 10u32.pow((NUM_SIZE - i - 1) as u32);
        } else if i >= INP_LEN - NUM_SIZE && i < INP_LEN {
            arr[i] = 10u32.pow((NUM_SIZE - (i - (INP_LEN - NUM_SIZE)) - 1) as u32);
        }
        i += 1;
    }
    Simd::from_array(arr)
}

#[target_feature(enable = "avx512f")]
unsafe fn simd_parse_inner<const INP_LEN: usize, const NUM_SIZE: usize, const LANE_SIZE: usize>(
    line: &[u8; INP_LEN],
) -> (u64, u64)
where
    LaneCount<LANE_SIZE>: SupportedLaneCount,
{
    let multipliers = const { get_multipliers::<LANE_SIZE, INP_LEN, NUM_SIZE>() };
    let mut line_padded = [0u8; LANE_SIZE];
    line_padded[..INP_LEN].copy_from_slice(line);
    let line = line_padded;

    let line = Simd::<u8, LANE_SIZE>::from_array(line);
    let digits = line - Simd::splat(b'0');
    let digits_big: Simd<u32, LANE_SIZE> = digits.cast() * multipliers;
    let left = digits_big.as_array().iter().take(NUM_SIZE).sum::<u32>() as u64;
    let right = digits_big
        .as_array()
        .iter()
        .skip(INP_LEN - NUM_SIZE)
        .sum::<u32>() as u64;

    (left, right)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simd() {
        assert_eq!(parse_line_simd(b"01234   56789", 5, 8), (1234, 56789));
    }
}
