const INPUT: &str = include_str!("inputs/1.txt");
use crate::util::FastParse;

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

pub fn part2() -> u64 {
    let (left, right): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(char::is_whitespace).unwrap();
            (
                u64::fast_parse(l).unwrap().0,
                u64::fast_parse(r.trim_ascii_start()).unwrap().0,
            )
        })
        .unzip();
    left.iter()
        .map(|&val| right.iter().filter(|&&v| v == val).count() as u64 * val)
        .sum()
}
