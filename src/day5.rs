// 113.4us
pub fn part1(input: &str) -> u64 {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let orderings = parse_orderings(ordering_rules);

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
        .map(|nums| nums.mid())
        .sum()
}

// 579.6us
pub fn part2(input: &str) -> u64 {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();
    let orderings = parse_orderings(ordering_rules);

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
        .map(|nums| nums.mid())
        .sum()
}

fn parse_orderings(input: &str) -> Vec<(u64, u64)> {
    input
        .split('\n')
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn parse_updates(input: &str) -> impl Iterator<Item = UpdatesMap> {
    input.trim_ascii_end().split('\n').map(|line| {
        let mut map = UpdatesMap([usize::MAX; 99]);
        for (pos, n) in line.split(',').enumerate() {
            let n = n.parse::<usize>().unwrap();
            map.0[n] = pos;
        }
        map
    })
}

// Maps the numbers 1->99 to their positions in the update list
struct UpdatesMap([usize; 99]);

impl UpdatesMap {
    pub fn position(&self, val: &u64) -> Option<usize> {
        Some(self.0[*val as usize]).filter(|v| *v != usize::MAX)
    }

    pub fn swap(&mut self, a_val: u64, b_val: u64) {
        let old_a = self.position(&a_val).unwrap();
        let old_b = self.position(&b_val).unwrap();
        self.0[a_val as usize] = old_b;
        self.0[b_val as usize] = old_a;
    }

    pub fn mid(&self) -> u64 {
        let mut numbers: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != usize::MAX)
            .map(|(num, pos)| (*pos, num as u64))
            .collect();
        numbers.sort_by_key(|(pos, _)| *pos);
        numbers[numbers.len() / 2].1
    }
}
