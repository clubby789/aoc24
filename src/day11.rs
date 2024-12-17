use either::Either;
use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> Either<u64, String> {
    let mut stones = input
        .trim_ascii_end()
        .split(' ')
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect::<FxHashMap<_, _>>();
    for _ in 0..25 {
        stones = blink(stones);
    }
    Either::Left(stones.values().sum())
}

pub fn part2(input: &str) -> Either<u64, String> {
    let mut stones = input
        .trim_ascii_end()
        .split(' ')
        .map(|n| (n.parse::<u64>().unwrap(), 1))
        .collect::<FxHashMap<_, _>>();
    for _ in 0..75 {
        stones = blink(stones);
    }
    Either::Left(stones.values().sum())
}

fn blink(stones: FxHashMap<u64, u64>) -> FxHashMap<u64, u64> {
    let mut new_stones = FxHashMap::default();
    new_stones.reserve(stones.len() * 2);
    for (&num, &count) in stones.iter() {
        if num == 0 {
            *new_stones.entry(1).or_default() += count;
        } else if num.ilog10() & 1 == 1 {
            let d = 10u64.pow((num.ilog10() + 1) / 2);
            let right = num % d;
            let left = (num - right) / d;
            *new_stones.entry(left).or_default() += count;
            *new_stones.entry(right).or_default() += count;
        } else {
            *new_stones.entry(num * 2024).or_default() += count;
        }
    }

    new_stones
}
