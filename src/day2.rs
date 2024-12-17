use std::cmp::Ordering;

use either::Either;

// 8.5us
pub fn part1(input: &str) -> Either<u64, String> {
    let mut input = input.as_bytes();
    let mut count = 0;

    while !input.is_empty() {
        let result;
        (input, result) = check_line_valid(input);
        if result.is_ok() {
            count += 1;
        }
    }
    Either::Left(count)
}

// 19.8s
pub fn part2(input: &str) -> Either<u64, String> {
    let mut input = input.as_bytes();
    let mut count = 0;
    let mut numbers = [0; 8];

    while !input.is_empty() {
        let (next_input, result) = check_line_valid(input);
        match result {
            Ok(_) => count += 1,
            Err(idx) => {
                let mut i = 0;
                for_each_num_on_line(input, |n| {
                    numbers[i] = n;
                    i += 1;
                    true
                });

                if check_sequence_valid_damped(&mut numbers[..i], idx) {
                    count += 1;
                }
            }
        }
        input = next_input;
    }
    Either::Left(count)
}

fn check_line_valid(mut input: &[u8]) -> (&[u8], Result<(), usize>) {
    #[derive(Default)]
    enum State {
        #[default]
        New,
        First(u8),
        InProgress(Ordering, u8),
    }

    let mut state = State::default();
    let mut checked = 0;
    let ok;
    (input, ok) = for_each_num_on_line(input, |num| match state {
        State::New => {
            state = State::First(num);
            true
        }
        State::First(prev) => {
            if matches!(num.abs_diff(prev), 1..=3) {
                state = State::InProgress(num.cmp(&prev), num);
                checked += 1;
                true
            } else {
                false
            }
        }
        State::InProgress(direction, ref mut prev) => {
            if matches!(num.abs_diff(*prev), 1..=3) && num.cmp(prev) == direction {
                *prev = num;
                checked += 1;
                true
            } else {
                false
            }
        }
    });

    if ok {
        (input, Ok(()))
    } else {
        (
            &input[memchr::memchr(b'\n', input).unwrap() + 1..],
            Err(checked),
        )
    }
}

fn check_sequence_valid_damped(nums: &mut [u8], idx: usize) -> bool {
    for i in idx.saturating_sub(1)..=(idx + 1) {
        let old = nums[i];
        nums.copy_within(i + 1.., i);
        if check_sequence_valid(&nums[..nums.len() - 1]).is_ok() {
            return true;
        }
        nums.copy_within(i..nums.len() - 2, i + 1);
        nums[i] = old;
    }
    false
}

fn check_sequence_valid(nums: &[u8]) -> Result<(), usize> {
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

        if f(num) {
            if last {
                return (rest, true);
            }
        } else {
            return (input, false);
        }
        input = rest;
    }
}
