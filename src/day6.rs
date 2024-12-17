use either::Either;

pub fn part1(input: &str) -> Either<u64, String> {
    let input = input.as_bytes();
    let start = memchr::memchr(b'^', input).unwrap();
    let line_length = memchr::memchr(b'\n', input).unwrap() + 1;

    let directions = [
        // Up
        -(line_length as isize),
        // Right
        1,
        // Down
        line_length as isize,
        // Left
        -1,
    ];

    let mut visited = vec![false; input.len()];
    let mut visit_count = 0;
    walk_grid(start, Direction::Up, &directions, input, |pos, _| {
        if !visited[pos] {
            visited[pos] = true;
            visit_count += 1;
        }
        true
    });
    Either::Left(visit_count)
}

pub fn part2(input: &str) -> Either<u64, String> {
    let input = input.as_bytes();
    let start = memchr::memchr(b'^', input).unwrap();
    let line_length = memchr::memchr(b'\n', input).unwrap() + 1;

    let directions = [
        // Up
        -(line_length as isize),
        // Right
        1,
        // Down
        line_length as isize,
        // Left
        -1,
    ];

    let mut second_grid = input.to_vec();
    let mut visited = vec![[false; 4]; input.len()];

    Either::Left(
        (0..input.len())
            .filter(|&i| input[i] == b'.')
            .filter(|&i| {
                second_grid[i] = b'#';

                let mut cycle = false;
                visited.fill([false; 4]);
                walk_grid(
                    start,
                    Direction::Up,
                    &directions,
                    &second_grid,
                    |pos, dir| {
                        if visited[pos][dir as usize] {
                            cycle = true;
                            false
                        } else {
                            visited[pos][dir as usize] = true;
                            true
                        }
                    },
                );

                second_grid[i] = b'.';

                cycle
            })
            .count() as u64,
    )
}

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(&mut self) {
        *self = self.rotated();
    }

    #[must_use]
    pub fn rotated(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn walk_grid<F>(
    mut pos: usize,
    mut direction: Direction,
    directions: &[isize; 4],
    input: &[u8],
    mut f: F,
) where
    F: FnMut(usize, Direction) -> bool,
{
    loop {
        if !f(pos, direction) {
            break;
        }
        let Some(next) = pos.checked_add_signed(directions[direction as usize]) else {
            break;
        };

        match input.get(next) {
            Some(b'#') => {
                direction.rotate();
                continue;
            }
            Some(b'\n') | None => {
                break;
            }
            Some(x) => {
                debug_assert!(matches!(x, b'.' | b'^'), "`{:?}`", *x as char);
                pos = next;
            }
        }
    }
}
