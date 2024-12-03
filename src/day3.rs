// 25.9us
pub fn part1(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut total = 0;
    while let Some(start) = memchr::memmem::find(input, b"mul(") {
        input = &input[start + 4..];
        if let Some((x, y, rest)) = parse_mul_body(input) {
            total += x * y;
            input = rest;
        }
    }
    total
}

// 37.2us
pub fn part2(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut total = 0;
    let mut enabled = true;
    while let Some(start) = memchr::memmem::find(input, b"mul(") {
        let to_check = &input[..start];
        let last_enable = memchr::memmem::rfind(to_check, b"do()");
        let last_disable = memchr::memmem::rfind(to_check, b"don't()");
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
        input = &input[start + 4..];
        if enabled {
            if let Some((x, y, rest)) = parse_mul_body(input) {
                total += x * y;
                input = rest;
            }
        }
    }
    total
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

// Given bytes after a `mul(`, return the X and Y values (if valid) and the bytes after the mul
fn parse_mul_body(input: &[u8]) -> Option<(u64, u64, &[u8])> {
    let (x, y, rest) = match input {
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
        ] => (parse_ascii!(x1, x2, x3), parse_ascii!(y1, y2, y3), rest),
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
        ] => (parse_ascii!(x1, x2, x3), parse_ascii!(y1, y2), rest),
        // (XXX,Y)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            x3 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2, x3), parse_ascii!(y1), rest),
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
        ] => (parse_ascii!(x1, x2), parse_ascii!(y1, y2, y3), rest),
        // (XX,YY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2), parse_ascii!(y1, y2), rest),
        // (XX,Y)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1, x2), parse_ascii!(y1), rest),
        // (X,YYY)
        [
            x1 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            y3 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1), parse_ascii!(y1, y2, y3), rest),
        // (X,YY)
        [
            x1 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            rest @ ..,
        ] => (parse_ascii!(x1), parse_ascii!(y1, y2), rest),
        // (X,Y)
        [x1 @ b'0'..=b'9', b',', y1 @ b'0'..=b'9', b')', rest @ ..] => {
            (parse_ascii!(x1), parse_ascii!(y1), rest)
        }
        _ => return None,
    };
    Some((x, y, rest))
}
