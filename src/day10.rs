// 121.0us
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let line_length = memchr::memchr(b'\n', input).unwrap() + 1;
    let directions = [-(line_length as isize), 1, line_length as isize, -1];

    memchr::memchr_iter(b'0', input)
        .map(|pos| {
            pathfinding::prelude::bfs_reach(pos, |&old_pos| successors(input, old_pos, &directions))
                .filter(|&n| input[n] == b'9')
                .count() as u64
        })
        .sum()
}

// 106.76us
pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let line_length = memchr::memchr(b'\n', input).unwrap() + 1;
    let directions = [-(line_length as isize), 1, line_length as isize, -1];

    memchr::memchr_iter(b'0', input)
        .map(|pos| {
            pathfinding::prelude::count_paths(
                pos,
                |&old_pos| successors(input, old_pos, &directions),
                |&fin| input[fin] == b'9',
            ) as u64
        })
        .sum()
}

fn successors(input: &[u8], from: usize, directions: &[isize; 4]) -> impl Iterator<Item = usize> {
    let from_val = input[from];
    directions.into_iter().filter_map(move |&off| {
        let new_pos = from.checked_add_signed(off)?;
        let new_val = *input.get(new_pos)?;
        if new_val == from_val + 1 {
            Some(new_pos)
        } else {
            None
        }
    })
}
