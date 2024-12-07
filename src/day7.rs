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
            let (test_num, nums) = line.split_once(": ").unwrap();
            let test_num = test_num.parse::<u64>().unwrap();
            let nums = nums
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if is_valid_rev::<ALLOW_CONCAT>(test_num, &nums) {
                Some(test_num)
            } else {
                None
            }
        })
        .sum()
}

fn is_valid_rev<const ALLOW_CONCAT: bool>(current: u64, nums: &[u64]) -> bool {
    let Some((&last, rest)) = nums.split_last() else {
        return current == 0;
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
    val.ilog10() + 1
}
