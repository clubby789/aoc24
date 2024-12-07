pub fn part1(input: &str) -> u64 {
    solve::<false>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<true>(input)
}

fn solve<const ALLOW_CONCAT: bool>(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            // Leaves a trailing space in `nums`, which the parser requires
            let (test_num, nums) = line.split_once(":").unwrap();
            let test_num = test_num
                .bytes()
                .fold(0, |acc, v| acc * 10 + (v - b'0') as u64);
            if is_valid_rev::<ALLOW_CONCAT>(test_num, &nums.as_bytes()) {
                Some(test_num as u64)
            } else {
                None
            }
        })
        .sum()
}

fn is_valid_rev<const ALLOW_CONCAT: bool>(current: u64, nums: &[u8]) -> bool {
    debug_assert!(nums.is_empty() || nums[0] == b' ');
    let (rest, last) = match nums {
        [rest @ .., b' ', a] => (rest, (*a - b'0') as u64),
        [rest @ .., b' ', a, b] => (rest, (*a - b'0') as u64 * 10 + (*b - b'0') as u64),
        [rest @ .., b' ', a, b, c] => (
            rest,
            (*a - b'0') as u64 * 100 + (*b - b'0') as u64 * 10 + (*c - b'0') as u64,
        ),
        _ => {
            return current == 0;
        }
    };

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
