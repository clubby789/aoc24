// 1.0ms
pub fn part1(input: &str) -> u64 {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let orderings: Vec<(u64, u64)> = ordering_rules
        .split('\n')
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    updates
        .trim_ascii_end()
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|nums| {
            orderings.iter().all(|(first, last)| {
                if let Some((first_pos, last_pos)) = Option::zip(
                    nums.iter().position(|n| n == first),
                    nums.iter().position(|n| n == last),
                ) {
                    first_pos < last_pos
                } else {
                    true
                }
            })
        })
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

// 5.85ms
pub fn part2(input: &str) -> u64 {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let orderings: Vec<(u64, u64)> = ordering_rules
        .split('\n')
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    updates
        .trim_ascii_end()
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(|mut nums| {
            let mut any_incorrect = false;

            let mut changed_in_loop = true;

            while changed_in_loop {
                changed_in_loop = false;
                orderings.iter().for_each(|(first, last)| {
                    if let Some((first_pos, last_pos)) = Option::zip(
                        nums.iter().position(|n| n == first),
                        nums.iter().position(|n| n == last),
                    ) {
                        if first_pos > last_pos {
                            any_incorrect = true;
                            changed_in_loop = true;
                            nums.swap(first_pos, last_pos);
                        }
                    }
                });
            }

            any_incorrect.then_some(nums)
        })
        .map(|nums| (&nums)[nums.len() / 2])
        .sum()
}
