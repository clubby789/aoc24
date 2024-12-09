use std::{fmt::Write, usize};

// 228.7us
pub fn part1(input: &str) -> u64 {
    let mut disk = parse_disk(input);
    let mut start = 0;
    let mut end = disk.len() - 1;
    while start < end {
        while disk[start] != Block::Empty {
            start += 1;
        }
        while disk[end] == Block::Empty {
            end -= 1;
        }
        if start >= end {
            break;
        }
        disk.swap(start, end);
    }
    disk_checksum_contiguous(&disk)
}

// 81.5ms
pub fn part2(input: &str) -> u64 {
    let mut disk = parse_disk(input);
    let mut end: usize = disk.len() - 1;
    let mut last_tried_id = usize::MAX;

    'outer: loop {
        while disk[end] == Block::Empty {
            end -= 1;
        }
        let file_end = end;
        let mut file_start = end;
        while disk[file_start - 1] == disk[file_end] {
            file_start -= 1;
            if file_start == 0 {
                break 'outer;
            }
        }
        match disk[file_start] {
            Block::File { id } if id >= last_tried_id => {
                end = file_start - 1;
                continue;
            }
            Block::File { id } if id == 0 => {
                break;
            }
            Block::File { id } => last_tried_id = id,
            Block::Empty => {
                #[cfg(debug_assertions)]
                unreachable!();
            }
        }
        debug_assert!(file_end >= file_start);
        let file_len = file_end - file_start;

        let mut empty_start = 0;
        let mut empty_end = 0;
        let mut ok = false;
        'find_empty: while empty_start < file_start {
            // Find empty block
            while disk[empty_start] != Block::Empty {
                empty_start += 1;
            }
            if empty_start >= file_start {
                break;
            }
            empty_end = empty_start;
            while disk[empty_end + 1] == Block::Empty {
                empty_end += 1;
                if empty_end + 1 >= disk.len() {
                    break 'find_empty;
                }
            }
            if empty_end - empty_start < file_len {
                empty_start = empty_end + 1;
            } else {
                ok = true;
                break;
            }
        }
        if ok {
            // Found enough free space
            debug_assert!(empty_end >= empty_start);
            debug_assert!(file_start > empty_end);
            for (empty, file) in (empty_start..=empty_end).zip(file_start..=file_end) {
                disk.swap(empty, file);
            }
        } else {
            end = file_start - 1;
        }
    }

    disk_checksum(&disk)
}

fn parse_disk(input: &str) -> Vec<Block> {
    let mut disk = Vec::with_capacity(input.len() * 9);
    for (i, b) in input.trim_ascii_end().bytes().enumerate() {
        let block = if i & 1 == 0 {
            Block::File { id: i / 2 }
        } else {
            Block::Empty
        };
        for _ in 0..(b - b'0') {
            assert!(disk.capacity() > disk.len());
            disk.push(block);
        }
    }
    disk
}

fn disk_checksum_contiguous(disk: &[Block]) -> u64 {
    disk.iter()
        .enumerate()
        .take_while(|(_, entry)| matches!(entry, Block::File { .. }))
        .map(|(pos, entry)| {
            let Block::File { id } = *entry else {
                unreachable!()
            };
            (pos * id) as u64
        })
        .sum()
}

fn disk_checksum(disk: &[Block]) -> u64 {
    disk.iter()
        .enumerate()
        .map(|(pos, entry)| {
            if let Block::File { id } = *entry {
                (pos * id) as u64
            } else {
                0
            }
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File { id: usize },
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_char('.'),
            Self::File { id } => f.write_fmt(format_args!("{id}")),
        }
    }
}
