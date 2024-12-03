// 26.4us
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

// 39.0us
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
    let (x, y) = match input {
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
            ..,
        ] => (parse_ascii!(x1, x2, x3), parse_ascii!(y1, y2, y3)),
        // (XXX,YY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            x3 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            ..,
        ] => (parse_ascii!(x1, x2, x3), parse_ascii!(y1, y2)),
        // (XXX,Y)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            x3 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            b')',
            ..,
        ] => (parse_ascii!(x1, x2, x3), parse_ascii!(y1)),
        // (XX,YYY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            y3 @ b'0'..=b'9',
            b')',
            ..,
        ] => (parse_ascii!(x1, x2), parse_ascii!(y1, y2, y3)),
        // (XX,YY)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            ..,
        ] => (parse_ascii!(x1, x2), parse_ascii!(y1, y2)),
        // (XX,Y)
        [
            x1 @ b'0'..=b'9',
            x2 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            b')',
            ..,
        ] => (parse_ascii!(x1, x2), parse_ascii!(y1)),
        // (X,YYY)
        [
            x1 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            y3 @ b'0'..=b'9',
            b')',
            ..,
        ] => (parse_ascii!(x1), parse_ascii!(y1, y2, y3)),
        // (X,YY)
        [
            x1 @ b'0'..=b'9',
            b',',
            y1 @ b'0'..=b'9',
            y2 @ b'0'..=b'9',
            b')',
            ..,
        ] => (parse_ascii!(x1), parse_ascii!(y1, y2)),
        // (X,Y)
        [x1 @ b'0'..=b'9', b',', y1 @ b'0'..=b'9', b')', ..] => {
            (parse_ascii!(x1), parse_ascii!(y1))
        }
        _ => return None,
    };
    // TODO: return the `..` part
    Some((x, y, input))
}
