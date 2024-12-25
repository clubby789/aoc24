use std::simd::{Simd, cmp::SimdPartialEq};

use either::Either;

const WIDTH: usize = 5;
const HEIGHT: usize = 7;
const GRID_SIZE_BYTES: usize = HEIGHT * (WIDTH + 1);
const SIMD_SIZE: usize = GRID_SIZE_BYTES.next_power_of_two();

type Lock = Simd<u8, SIMD_SIZE>;
type Key = Simd<u8, SIMD_SIZE>;

// 43.4us
pub fn part1(input: &str) -> Either<u64, String> {
    let hash_mask: Simd<u8, SIMD_SIZE> = Simd::splat(b'#');
    let (locks, keys): (Vec<Lock>, Vec<Key>) = input
        .as_bytes()
        .chunks(GRID_SIZE_BYTES + 1)
        .map(|chunk| {
            let (chunk, _) = chunk.split_at(GRID_SIZE_BYTES);
            Lock::load_or_default(chunk)
        })
        .partition(|chunk| chunk[0] == b'#');
    Either::Left(
        locks
            .iter()
            .flat_map(|lock| {
                keys.iter().filter(move |&key| {
                    let lock_hashes = lock.simd_eq(hash_mask);
                    let key_hashes = key.simd_eq(hash_mask);
                    !((lock_hashes & key_hashes).any())
                })
            })
            .count() as u64,
    )
}

pub fn part2(_: &str) -> Either<u64, String> {
    Either::Left(0)
}
