use std::simd::u16x16;

pub fn part1(input: &str) -> u64 {
    solve::<false>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<true>(input)
}

fn solve<const ALLOW_CONCAT: bool>(input: &str) -> u64 {
    let mut start = 0;
    let input = input.as_bytes();
    memchr::memchr_iter(b'\n', input)
        .filter_map(|end| {
            let line = &input[start..end];
            start = end + 1;

            let (test_num, bitset, len) = parse_line(line);
            if is_valid_rev::<ALLOW_CONCAT>(test_num, bitset, len) {
                Some(test_num)
            } else {
                None
            }
        })
        .sum()
}

fn parse_line(mut line: &[u8]) -> (u64, u16x16, usize) {
    let mut test_num = 0;
    let mut arr = [0; 16];
    while line[0] != b':' {
        test_num *= 10;
        test_num += (line[0] - b'0') as u64;
        line = &line[1..];
    }
    line = &line[1..];
    let mut i = 0;
    for &b in line.iter() {
        if b == b' ' {
            i += 1;
        } else {
            arr[i] *= 10;
            arr[i] += (b - b'0') as u16;
        }
    }

    let mut bitset = u16x16::from_array(arr);
    // Shift the elements until the last number is at the end
    while bitset.as_array()[15] == 0 {
        bitset = bitset.rotate_elements_right::<1>();
    }
    (test_num, bitset, i)
}

fn is_valid_rev<const ALLOW_CONCAT: bool>(current: u64, nums: u16x16, remaining: usize) -> bool {
    if remaining == 0 {
        return current == 0;
    }
    let (last, rest) = shift(nums);

    let last = last as u64;
    if let Some(subbed) = current.checked_sub(last) {
        if is_valid_rev::<ALLOW_CONCAT>(subbed, rest, remaining - 1) {
            return true;
        }
    }

    if ALLOW_CONCAT && !(last > current) {
        let divisor = 10u64.pow(ndigits(last));
        if (current - last) % divisor == 0 {
            if is_valid_rev::<ALLOW_CONCAT>((current - last) / divisor, rest, remaining - 1) {
                return true;
            }
        }
    }

    let (div, rem) = (current / last, current % last);
    if rem == 0 {
        return is_valid_rev::<ALLOW_CONCAT>(div, rest, remaining - 1);
    }
    false
}

fn ndigits(val: u64) -> u32 {
    match val {
        0..=9 => 1,
        10..=99 => 2,
        _ => {
            debug_assert!(matches!(val, 100..=999));
            3
        }
    }
}

fn shift(set: u16x16) -> (u16, u16x16) {
    let val = set.as_array()[15];
    let rotated = set.rotate_elements_right::<1>();
    (val, rotated)
}
