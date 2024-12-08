use rustc_hash::FxHashSet;


// 4.33ms
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let line_length = input.iter().copied().position(|b| b == b'\n').unwrap() + 1;
    let idx_to_pos = |idx: usize| {
        let row = idx / line_length;
        let col = idx % line_length;
        (row, col)
    };
    let pos_to_idx = |row: usize, col: usize| {
        debug_assert!(col < line_length);
        row * line_length + col
    };

    let mut antinodes = 0;
    'outer: for antinode_pos in (0..input.len()).filter(|&i| input[i] != b'\n') {
        let (y1, x1) = idx_to_pos(antinode_pos);
        for antennae_pos in
            (0..input.len()).filter(|&j| input[j].is_ascii_alphanumeric() && antinode_pos != j)
        {
            let c = input[antennae_pos];
            let (y2, x2) = idx_to_pos(antennae_pos);
            let dx = x2 as isize - x1 as isize;
            let dy = y2 as isize - y1 as isize;
            let Some(x3) = x2.checked_add_signed(dx) else {
                continue;
            };
            if x3 >= line_length {
                continue;
            }
            let Some(y3) = y2.checked_add_signed(dy) else {
                continue;
            };
            if input.get(pos_to_idx(y3, x3)) == Some(&c) {
                antinodes += 1;
                continue 'outer;
            }
        }
    }

    antinodes
}

// 203.5us
pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let line_length = input.iter().copied().position(|b| b == b'\n').unwrap() + 1;
    let idx_to_pos = |idx: usize| {
        let row = idx / line_length;
        let col = idx % line_length;
        (row, col)
    };

    let mut antinodes = FxHashSet::default();
    for antennae_a_pos in
        (0..input.len()).filter(|&antennae_a_pos| input[antennae_a_pos].is_ascii_alphanumeric())
    {
        let (y1, x1) = idx_to_pos(antennae_a_pos);
        for antennae_b_pos in (0..input.len()).filter(|&antennae_b_pos| {
            input[antennae_b_pos] == input[antennae_a_pos] && antennae_a_pos != antennae_b_pos
        }) {
            let (y2, x2) = idx_to_pos(antennae_b_pos);
            let dx = x2 as isize - x1 as isize;
            let dy = y2 as isize - y1 as isize;
            debug_assert_ne!(dy, 0);
            debug_assert_ne!(dx, 0);

            let step_x = dx / gcd(dx.abs() as usize, dy.abs() as usize) as isize;
            let step_y = dy / gcd(dx.abs() as usize, dy.abs() as usize) as isize;
            // Forward
            let (mut x, mut y) = (x1, y1);
            while (0..line_length - 1).contains(&x) && (0..line_length - 1).contains(&y) {
                antinodes.insert((y, x));
                let Some(nx) = x.checked_add_signed(step_x) else {
                    break;
                };
                let Some(ny) = y.checked_add_signed(step_y) else {
                    break;
                };
                x = nx;
                y = ny;
            }
        }
    }

    antinodes.len() as u64
}

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
