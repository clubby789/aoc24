pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (test_num, nums) = line.split_once(": ").unwrap();
            let test_num = test_num.parse::<u64>().unwrap();
            let nums = nums
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if is_valid::<false>(test_num, 0, &nums) {
                Some(test_num)
            } else {
                None
            }
        })
        .sum()
}

fn is_valid<const ALLOW_CONCAT: bool>(target: u64, current: u64, remaining: &[u64]) -> bool {
    if current > target {
        return false;
    }
    let Some((&next, rest)) = remaining.split_first() else {
        return target == current;
    };
    if is_valid::<ALLOW_CONCAT>(target, current + next, rest)
        || is_valid::<ALLOW_CONCAT>(target, current * next, rest)
    {
        true
    } else if ALLOW_CONCAT {
        let mut pow = 10;
        while next > pow {
            pow *= 10;
        }
        let new = current * pow + next;
        is_valid::<ALLOW_CONCAT>(target, new, rest)
    } else {
        false
    }
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (test_num, nums) = line.split_once(": ").unwrap();
            let test_num = test_num.parse::<u64>().unwrap();
            let nums = nums
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if is_valid::<true>(test_num, 0, &nums) {
                Some(test_num)
            } else {
                None
            }
        })
        .sum()
}
