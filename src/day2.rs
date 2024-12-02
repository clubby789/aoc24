use crate::util::FastParse;

// 8.5us
pub fn part1(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut count = 0;

    let mut prev: Option<u8> = None;
    let mut direction = None;
    while !input.is_empty() {
        let done;
        (input, done) = for_each_num_on_line(input, |num| {
            if let Some(prev_val) = prev {
                if !matches!(prev_val.abs_diff(num), 1..=3) {
                    input = &input[memchr::memchr(b'\n', input).unwrap() + 1..];
                    prev = None;
                    direction = None;
                    false
                } else if let Some(direction_val) = direction {
                    if direction_val != num.cmp(&prev_val) {
                        input = &input[memchr::memchr(b'\n', input).unwrap() + 1..];
                        prev = None;
                        direction = None;
                        false
                    } else {
                        prev = Some(num);
                       true
                    }
                } else {
                    prev = Some(num);
                    direction = Some(num.cmp(&prev_val));
                    true
                }
            } else {
                prev = Some(num);
                true
            }
        });

        prev = None;
        direction = None;
        if done {
            count += 1;
        } else {
            input = &input[memchr::memchr(b'\n', input).unwrap() + 1..];
        }
    }
    count
}

// 39.1us
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let mut nums = Vec::with_capacity(line.len() / 2);
            nums.extend(line.split_ascii_whitespace().map(u64::fast_parse_unchecked));
            match check_sequence_valid(&nums) {
                Ok(_) => true,
                Err(idx) => {
                    // Push and pop from the same vec to retain the storage
                    for i in idx.saturating_sub(2)..(idx + 2).min(nums.len()) {
                        let old = nums.remove(i);
                        if check_sequence_valid(&nums).is_ok() {
                            return true;
                        }
                        nums.insert(i, old);
                    }
                    false
                }
            }
        })
        .count() as _
}

fn check_sequence_valid(nums: &[u64]) -> Result<(), usize> {
    let direction = nums[1].cmp(&nums[0]);
    for i in 0..nums.len() - 1 {
        let n1 = nums[i];
        let n2 = nums[i + 1];
        match n2.abs_diff(n1) {
            1..=3 if n2.cmp(&n1) == direction => continue,
            _ => return Err(i),
        }
    }
    Ok(())
}

fn for_each_num_on_line<F: FnMut(u8) -> bool>(mut input: &[u8], mut f: F) -> (&[u8], bool) {
    loop {
        let (num, last, rest) = match input {
            [hi, b' ', rest @ ..] => (hi - b'0', false, rest),
            [hi, lo, b' ', rest @ ..] => ((hi - b'0') * 10 + lo - b'0', false, rest),
            [hi, b'\n', rest @ ..] => (hi - b'0', true, rest),
            [hi, lo, b'\n', rest @ ..] => ((hi - b'0') * 10 + lo - b'0', true, rest),
            _ => unreachable!("{:?}", input),
        };

        input = rest;
        if f(num) {
            if last {
                return (input, true);
            }
        } else {
            return (input, false);
        }
    }
}
