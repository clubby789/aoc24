use memchr::memmem;

// 5.1us
pub fn part1(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut total = 0;

    while let Some(start) = memchr::memchr(b'm', input) {
        input = &input[start + 1..];
        match input {
            [b'u', b'l', b'(', rest @ ..] => {
                let val;
                (val, input) = parse_mul_body(rest);
                total += val;
            }
            _ => continue,
        }
    }
    total
}

// 12.0us
pub fn part2(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut total = 0;
    let mut enabled = true;
    let enable_finder = memmem::FinderRev::new(b"do()");
    let disable_finder = memmem::FinderRev::new(b"don't()");

    while let Some(start) = memchr::memchr(b'm', input) {
        let to_check = &input[..start];
        let (val, new_input) = match &input[start + 1..] {
            [b'u', b'l', b'(', rest @ ..] => parse_mul_body(rest),
            rest => {
                enabled = enabled_after_block(enabled, &enable_finder, &disable_finder, &to_check);
                input = rest;
                continue;
            }
        };
        enabled = enabled_after_block(enabled, &enable_finder, &disable_finder, &to_check);
        input = new_input;
        if enabled {
            total += val;
        }
    }
    total
}

#[inline]
fn enabled_after_block(
    mut enabled: bool,
    enable_finder: &memmem::FinderRev,
    disable_finder: &memmem::FinderRev,
    block: &[u8],
) -> bool {
    let last_enable = enable_finder.rfind(block);
    let last_disable = disable_finder.rfind(block);
    if enabled {
        // If there is a disable, and there is no enable OR the last enable is before the last disable:
        if last_disable.is_some_and(|last_disable_pos| {
            last_enable.is_none_or(|last_enable_pos| last_enable_pos < last_disable_pos)
        }) {
            enabled = false;
        }
    } else {
        // If there is a enable, and there is no disable OR the last disable is before the last enable:
        if last_enable.is_some_and(|last_enable_pos| {
            last_disable.is_none_or(|last_disable_pos| last_disable_pos < last_enable_pos)
        }) {
            enabled = true;
        }
    }
    enabled
}

macro_rules! parse_ascii {
    ($lo:ident) => {
        (*$lo - b'0') as u64
    };
    ($mid:ident, $lo:ident) => {
        (*$mid - b'0') as u64 * 10 + parse_ascii!($lo)
    };
    ($hi:ident, $mid:ident, $lo:ident) => {
        (*$hi - b'0') as u64 * 100 + parse_ascii!($mid, $lo)
    };
}

fn parse_mul_body(input: &[u8]) -> (u64, &[u8]) {
    let (val, rest) = match input {
        // (XXX,YYY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            x3 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            y3 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2, x3) * parse_ascii!(y1, y2, y3), rest),
        // (XXX,YY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            x3 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2, x3) * parse_ascii!(y1, y2), rest),
        // (XXX,Y)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            x3 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2, x3) * parse_ascii!(y1), rest),
        // (XX,YYY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            y3 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2) * parse_ascii!(y1, y2, y3), rest),
        // (XX,YY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2) * parse_ascii!(y1, y2), rest),
        // (XX,Y)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2) * parse_ascii!(y1), rest),
        // (X,YYY)
        [
            x1 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            y3 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1) * parse_ascii!(y1, y2, y3), rest),
        // (X,YY)
        [
            x1 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1) * parse_ascii!(y1, y2), rest),
        // (X,Y)
        [x1 @ b'0'..=b'9', b',', y1 @ b'0'..=b'9', b')', rest @ ..] => {
            (parse_ascii!(x1) * parse_ascii!(y1), rest)
        }
        _ => return (0, input),
    };
    (val, rest)
}
