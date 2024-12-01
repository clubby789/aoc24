const INPUT: &str = include_str!("inputs/1.txt");

pub fn part1() -> u64 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(char::is_whitespace).unwrap();
            (
                l.trim_ascii().parse::<u64>().unwrap(),
                r.trim_ascii().parse::<u64>().unwrap(),
            )
        })
        .unzip();
    left.sort();
    right.sort();
    left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum()
}

pub fn part2() -> u64 {
    let (left, right): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(char::is_whitespace).unwrap();
            (
                l.trim_ascii().parse::<u64>().unwrap(),
                r.trim_ascii().parse::<u64>().unwrap(),
            )
        })
        .unzip();
    left.iter()
        .map(|&val| right.iter().filter(|&&v| v == val).count() as u64 * val)
        .sum()
}
