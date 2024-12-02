use std::cmp::Ordering;

use crate::util::FastParse;

// 7.5us
pub fn part1(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut count = 0;

    #[derive(Default)]
    enum State {
        #[default]
        New,
        First(u8),
        InProgress(Ordering, u8),
    }

    while !input.is_empty() {
        let mut state = State::default();
        let ok;
        (input, ok) = for_each_num_on_line(input, |num| match state {
            State::New => {
                state = State::First(num);
                true
            }
            State::First(prev) => {
                if matches!(num.abs_diff(prev), 1..=3) {
                    state = State::InProgress(num.cmp(&prev), num);
                    true
                } else {
                    false
                }
            }
            State::InProgress(direction, ref mut prev) => {
                if matches!(num.abs_diff(*prev), 1..=3) && num.cmp(prev) == direction {
                    *prev = num;
                    true
                } else {
                    false
                }
            }
        });

        if ok {
            count += 1;
        } else {
            input = &input[memchr::memchr(b'\n', input).unwrap() + 1..];
        }
    }
    count
}

// 40.0us
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let mut nums = Vec::with_capacity(line.len() / 2);
            nums.extend(line.split_ascii_whitespace().map(u64::fast_parse_unchecked));
            match check_sequence_valid(&nums) {
                Ok(_) => true,
                Err(idx) => {
                    // Outline as rarely called
                    check_sequence_valid_damped(nums, idx)
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

#[cold]
#[inline(never)]
fn check_sequence_valid_damped(mut nums: Vec<u64>, idx: usize) -> bool {
    for i in idx.saturating_sub(2)..(idx + 2) {
        let old = nums.remove(i);
        if check_sequence_valid(&nums).is_ok() {
            return true;
        }
        nums.insert(i, old);
    }
    false
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
