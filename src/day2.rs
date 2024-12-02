// 31.4.3us
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let mut nums = Vec::with_capacity(line.len() / 2);
            nums.extend(
                line.split_ascii_whitespace()
                    .map(|num| num.parse::<u64>().ok().unwrap()),
            );
            check_sequence_valid(&nums)
        })
        .count() as _
}

// 65.6us
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let mut nums = Vec::with_capacity(line.len() / 2);
            nums.extend(
                line.split_ascii_whitespace()
                    .map(|num| num.parse::<u64>().ok().unwrap()),
            );
            if check_sequence_valid(&nums) {
                true
            } else {
                for i in 0..nums.len() {
                    let mut check = nums.clone();
                    check.remove(i);
                    if check_sequence_valid(&check) {
                        return true;
                    }
                }
                false
            }
        })
        .count() as _
}

fn check_sequence_valid(nums: &[u64]) -> bool {
    let direction = nums[1].cmp(&nums[0]);
    nums.windows(2).all(move |window| {
        let n1 = window[0];
        let n2 = window[1];
        match n2.abs_diff(n1) {
            1..=3 if n2.cmp(&n1) == direction => true,
            _ => false,
        }
    })
}
