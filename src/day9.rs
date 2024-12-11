use std::{fmt::Write, num::NonZeroU64, usize};

// 228.7us
pub fn part1(input: &str) -> u64 {
    let mut disk = parse_disk_blocks(input);
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

// 16.0ms
pub fn part2(input: &str) -> u64 {
    let mut disk = parse_disk_segments(input);
    let mut last_block = u64::MAX;
    let mut back = disk.len() - 1;

    'outer: loop {
        while matches!(disk[back], Segment::Empty { .. }) {
            let Some(nb) = back.checked_sub(1) else {
                break 'outer;
            };
            back = nb;
        }
        let Segment::File { id, .. } = disk[back] else {
            unreachable!()
        };
        if id >= last_block {
            back -= 1;
            continue;
        } else {
            last_block = id;
        }

        // We only need to decrement `back` if new space wasn't inserted
        if !disk.move_file_earlier(back) {
            let Some(nb) = back.checked_sub(1) else {
                break;
            };
            back = nb;
        }
    }

    disk.checksum()
}

fn parse_disk_blocks(input: &str) -> Vec<Block> {
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

#[derive(Clone, Copy)]
enum Segment {
    File { id: u64, size: NonZeroU64 },
    Empty { size: NonZeroU64 },
}

impl Segment {
    #[must_use]
    pub fn insert_file(&mut self, id: u64, file_size: NonZeroU64) -> Option<Self> {
        let Segment::Empty { size } = self else {
            panic!("insert into file");
        };

        let Some(remaining) = size.get().checked_sub(file_size.get()) else {
            panic!("insert into too small space");
        };
        *self = Segment::File {
            id,
            size: file_size,
        };
        NonZeroU64::new(remaining).map(|size| Segment::Empty { size })
    }
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Segment::File { id, size } => {
                for _ in 0..size.get() {
                    write!(f, "{id}")?
                }
                Ok(())
            }
            Segment::Empty { size } => {
                for _ in 0..size.get() {
                    write!(f, ".")?
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
struct SegmentDisk {
    segments: Vec<Segment>,
}

impl SegmentDisk {
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    // Returns true if new space was inserted
    pub fn move_file_earlier(&mut self, file_idx: usize) -> bool {
        let Segment::File { id, size } = self[file_idx] else {
            panic!("non-file passed to `move_file_earlier`")
        };
        let Some(space) = self
            .segments
            .iter()
            .take(file_idx)
            .position(|seg| match seg {
                Segment::Empty { size: empty_size } => *empty_size >= size,
                _ => false,
            })
        else {
            return false;
        };

        self.segments[file_idx] = Segment::Empty { size };
        if let Some(new_space) = self.segments[space].insert_file(id, size) {
            self.segments.insert(space + 1, new_space);
            true
        } else {
            false
        }
    }

    pub fn checksum(&self) -> u64 {
        self.segments
            .iter()
            .fold((0, 0), |(total, block_pos), seg| {
                match seg {
                    Segment::File { id, size } => {
                        let size = size.get();
                        let val = id * (block_pos * size + ((size - 1) * size) / 2);
                        (total + val, block_pos + size)
                    }
                    Segment::Empty { size } => (total, block_pos + size.get()),
                }
            })
            .0
    }
}

impl std::ops::Index<usize> for SegmentDisk {
    type Output = Segment;

    fn index(&self, index: usize) -> &Self::Output {
        &self.segments[index]
    }
}

fn parse_disk_segments(input: &str) -> SegmentDisk {
    let mut segments = Vec::with_capacity(input.len());
    for (i, b) in input.trim_ascii_end().bytes().enumerate() {
        let Some(size) = NonZeroU64::new((b - b'0') as u64) else {
            continue;
        };
        if i & 1 == 0 {
            segments.push(Segment::File {
                id: i as u64 / 2,
                size,
            });
        } else {
            segments.push(Segment::Empty { size });
        };
    }
    SegmentDisk { segments }
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
