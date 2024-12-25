use either::Either;

const WIDTH: usize = 5;
const HEIGHT: usize = 7;
const GRID_SIZE_BYTES: usize = HEIGHT * (WIDTH + 1);

type Lock = [u8; GRID_SIZE_BYTES];
type Key = [u8; GRID_SIZE_BYTES];

pub fn part1(input: &str) -> Either<u64, String> {
    let (locks, keys): (Vec<Lock>, Vec<Key>) = input
        .as_bytes()
        .chunks(GRID_SIZE_BYTES + 1)
        .map(|chunk| {
            let (chunk, _) = chunk.split_at(GRID_SIZE_BYTES);
            <[u8; GRID_SIZE_BYTES]>::try_from(chunk).unwrap()
        })
        .partition(|chunk| chunk[0] == b'#');
    Either::Left(
        locks
            .iter()
            .flat_map(|lock| {
                keys.iter().filter(|key| {
                    lock.iter()
                        .zip(key.iter())
                        .all(|(&l, &k)| !(l == b'#' && k == b'#'))
                })
            })
            .count() as u64,
    )
}

pub fn part2(_: &str) -> Either<u64, String> {
    Either::Left(0)
}
