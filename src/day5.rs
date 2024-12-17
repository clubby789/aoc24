use either::Either;

// 72.5s
pub fn part1(input: &str) -> Either<u64, String> {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let orderings = parse_orderings(ordering_rules);

    Either::Left(
        parse_updates(updates)
            .filter(|nums| {
                orderings.iter().all(|(first, last)| {
                    if let Some((first_pos, last_pos)) =
                        Option::zip(nums.position(first), nums.position(last))
                    {
                        first_pos < last_pos
                    } else {
                        true
                    }
                })
            })
            .map(|nums| nums.mid() as u64)
            .sum(),
    )
}

// 483.8us
pub fn part2(input: &str) -> Either<u64, String> {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let orderings = parse_orderings(ordering_rules);

    Either::Left(
        parse_updates(updates)
            .filter_map(|mut nums| {
                let mut any_incorrect = false;

                let mut changed_in_loop = true;

                while changed_in_loop {
                    changed_in_loop = false;
                    orderings.iter().for_each(|(first, last)| {
                        if let Some((first_pos, last_pos)) =
                            Option::zip(nums.position(first), nums.position(last))
                        {
                            if first_pos > last_pos {
                                any_incorrect = true;
                                changed_in_loop = true;
                                nums.swap(*first, *last);
                            }
                        }
                    });
                }

                any_incorrect.then_some(nums)
            })
            .map(|nums| nums.mid() as u64)
            .sum(),
    )
}

fn parse_orderings(input: &str) -> Vec<(u8, u8)> {
    input
        .split('\n')
        .map(|line| match line.as_bytes() {
            &[a1, a2, p, b1, b2] => {
                debug_assert_eq!(p, b'|');
                (
                    (a1 - b'0') * 10 + (a2 - b'0'),
                    (b1 - b'0') * 10 + (b2 - b'0'),
                )
            }
            _ => unreachable!(),
        })
        .collect()
}

fn parse_updates(input: &str) -> impl Iterator<Item = UpdatesMap> + '_ {
    let input = input.as_bytes();
    let mut start = 0;
    memchr::memchr_iter(b'\n', input).map(move |end| {
        let mut map = UpdatesMap([u8::MAX; 256]);
        for (pos, chunk) in input[start..end].chunks(3).enumerate() {
            assert!(chunk.len() >= 2);
            debug_assert!(chunk[0].is_ascii_digit(), "`{:?}`", chunk[0] as char);
            debug_assert!(chunk[1].is_ascii_digit(), "`{:?}`", chunk[1] as char);
            let num = (chunk[0] - b'0') as usize * 10 + (chunk[1] - b'0') as usize;
            map.0[num] = pos as u8;
        }
        start = end + 1;
        map
    })
}

// Maps the numbers 1->99 to their positions in the update list
struct UpdatesMap([u8; 256]);

impl UpdatesMap {
    pub fn position(&self, val: &u8) -> Option<usize> {
        Some(self.0[*val as usize])
            .filter(|v| *v != u8::MAX)
            .map(|v| v as usize)
    }

    pub fn swap(&mut self, a_val: u8, b_val: u8) {
        let old_a = self.position(&a_val).unwrap();
        let old_b = self.position(&b_val).unwrap();
        self.0[a_val as usize] = old_b as u8;
        self.0[b_val as usize] = old_a as u8;
    }

    pub fn mid(&self) -> u8 {
        let mut numbers: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != u8::MAX)
            .map(|(num, pos)| (*pos, num as u8))
            .collect();
        numbers.sort_by_key(|(pos, _)| *pos);
        numbers[numbers.len() / 2].1
    }
}
