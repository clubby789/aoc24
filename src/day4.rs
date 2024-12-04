// 26.0us
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let line_length = input.iter().position(|b| *b == b'\n').unwrap() + 1;
    let offsets = [
        // Forward
        1,
        // Backward
        -1,
        // Down
        line_length as isize,
        // Up
        -(line_length as isize),
        // Up-Right
        -(line_length as isize) + 1,
        // Down-Right
        line_length as isize + 1,
        // Down-Left
        line_length as isize - 1,
        // Up-Left
        -(line_length as isize) - 1,
    ];

    (0..input.len())
        .filter(|&i| input[i] == b'X')
        .map(|i| {
            offsets
                .into_iter()
                .filter_map(|offset| {
                    // Iterate backward as if the last letter is OOB we can fail fast
                    for (n, letter) in b"MAS".iter().enumerate().rev() {
                        if input.get(i.checked_add_signed(offset * (n + 1) as isize)?)? != letter {
                            return None;
                        }
                    }
                    Some(())
                })
                .count()
        })
        .sum::<usize>() as u64
}

// 15.1us
pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let line_length = input.iter().position(|b| *b == b'\n').unwrap() + 1;
    let offsets = [
        // Up-Right
        -(line_length as isize) + 1,
        // Down-Right
        line_length as isize + 1,
        // Down-Left
        line_length as isize - 1,
        // Up-Left
        -(line_length as isize) - 1,
    ];

    (0..input.len())
        .filter(|&i| input[i] == b'A')
        .filter_map(|i| {
            let up_right = *input.get(i.checked_add_signed(offsets[0])?)?;
            let down_right = *input.get(i.checked_add_signed(offsets[1])?)?;
            let down_left = *input.get(i.checked_add_signed(offsets[2])?)?;
            let up_left = *input.get(i.checked_add_signed(offsets[3])?)?;
            match (up_left, down_right) {
                (b'M', b'S') | (b'S', b'M') => (),
                _ => return None,
            }
            match (up_right, down_left) {
                (b'M', b'S') | (b'S', b'M') => (),
                _ => return None,
            }
            Some(())
        })
        .count() as u64
}
