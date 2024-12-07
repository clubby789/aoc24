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

            let (test_num, bitset) = parse_line(line);
            if is_valid_rev::<ALLOW_CONCAT>(test_num, bitset) {
                Some(test_num)
            } else {
                None
            }
        })
        .sum()
}

fn parse_line(mut line: &[u8]) -> (u64, u16x16) {
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
    (test_num, bitset)
}

fn is_valid_rev<const ALLOW_CONCAT: bool>(current: u64, nums: u16x16) -> bool {
    if nums == u16x16::splat(0) {
        return current == 0;
    }
    let (last, rest) = shift(nums);

    let last = last as u64;
    if let Some(subbed) = current.checked_sub(last) {
        if is_valid_rev::<ALLOW_CONCAT>(subbed, rest) {
            return true;
        }
    }

    if ALLOW_CONCAT && !(last > current) {
        let divisor = 10u64.pow(ndigits(last));
        if (current - last) % divisor == 0 {
            if is_valid_rev::<ALLOW_CONCAT>((current - last) / divisor, rest) {
                return true;
            }
        }
    }

    let (div, rem) = (current / last, current % last);
    if rem == 0 {
        if is_valid_rev::<ALLOW_CONCAT>(div, rest) {
            return true;
        }
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
    #[cfg(debug_assertions)]
    {
        let mut zeroes = true;
        let set = set.as_array();
        assert_eq!(set[0], 0);
        for s in set {
            if zeroes {
                if *s != 0 {
                    zeroes = false;
                }
            } else {
                debug_assert_ne!(*s, 0);
            }
        }
    }
    let val = set.as_array()[15];
    let mut rotated = set.rotate_elements_right::<1>();
    rotated.as_mut_array()[0] = 0;
    (val, rotated)
}
