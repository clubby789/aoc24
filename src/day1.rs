const INPUT: &str = include_str!("inputs/1.txt");
const _: () = assert!(INPUT.as_bytes()[INPUT.len() - 1] == b'\n');

use rustc_hash::FxHashMap;

use crate::util::FastParse;

// 25.2us
pub fn part1() -> u64 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(char::is_whitespace).unwrap();
            (
                u64::fast_parse(l).unwrap().0,
                u64::fast_parse(r.trim_ascii_start()).unwrap().0,
            )
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum()
}

// 20.11us
pub fn part2() -> u64 {
    #[derive(Default)]
    struct Num {
        count: u64,
        appeared_left: bool,
    }
    let mut nums: FxHashMap<u64, Num> = FxHashMap::default();
    let mut input = INPUT;
    while !input.is_empty() {
        let (num1, len) = u64::fast_parse(input).unwrap();
        input = &input[len..].trim_ascii_start();
        nums.entry(num1).or_default().appeared_left = true;
        let (num2, len) = u64::fast_parse(input).unwrap();
        input = &input[len + 1..];
        nums.entry(num2).or_default().count += 1;
    }
    nums.iter()
        .filter(|(_, num)| num.appeared_left)
        .map(|(val, num)| val * num.count)
        .sum()
}
