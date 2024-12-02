use crate::util::FastParse;

// 32.8us
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let mut nums = Vec::with_capacity(line.len() / 2);
            nums.extend(line.split_ascii_whitespace().map(u64::fast_parse_unchecked));
            check_sequence_valid(&nums).is_ok()
        })
        .count() as _
}

// 45.0us
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
                    let mut check = nums.clone();
                    for i in idx.saturating_sub(2)..(idx + 2).min(nums.len()) {
                        let old = check.remove(i);
                        if check_sequence_valid(&check).is_ok() {
                            return true;
                        }
                        check.insert(i, old);
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
