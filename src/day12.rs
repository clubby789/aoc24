use either::Either;

pub fn part1(input: &str) -> Either<u64, String> {
    #[derive(Copy, Clone, Debug)]
    struct Region {
        // plant: char,
        perimeter: u64,
        area: u64,
    }

    let input = input.as_bytes();
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
    // List of regions
    let mut regions = vec![];
    // For each input cell, track twhether it has been grouped into a region
    let mut visited = vec![false; input.len()];

    while let Some((start, _)) = visited
        .iter()
        .enumerate()
        // Find cells which are not newlines
        .filter(|(i, _)| input[*i] != b'\n')
        // Then find the first which is not yet in a region
        .find(|(_, c)| !*c)
    {
        let plant = input[start];
        visited[start] = true;
        let mut region = Region {
            // plant: plant as char,
            area: 0,
            perimeter: 0,
        };
        let mut func = |neighbour: usize| {
            region.area += 1;
            // Add 1 to perimeter for each side that doesn't border a different cell type
            for dir in directions.iter() {
                if let Some(side) = neighbour.checked_add_signed(*dir) {
                    // Either we are bordering edge and go oob, or bordering a different plant
                    if input.get(side).map_or(true, |&v| v != plant) {
                        // println!("{side} is different, adding");
                        region.perimeter += 1;
                    }
                } else {
                    // We must be on the edge
                    // println!("on the edge, adding");
                    region.perimeter += 1;
                }
            }
        };
        func(start);
        flood_fill(input, visited.as_mut_slice(), start, &directions, &mut func);
        regions.push(region);
    }
    Either::Left(regions
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum())
}

// TODO
pub fn part2(_: &str) -> Either<u64, String> {
    Either::Left(0)
}

fn flood_fill<F>(
    input: &[u8],
    visited: &mut [bool],
    start: usize,
    directions: &[isize; 4],
    f: &mut F,
) where
    F: FnMut(usize),
{
    // println!("filling from {start}");
    for neighbour in directions
        .iter()
        .filter_map(|&o| start.checked_add_signed(o))
        .filter(|&p| input.get(p).is_some_and(|&v| v == input[start]))
    {
        if !visited[neighbour] {
            visited[neighbour] = true;
            f(neighbour);
            flood_fill(input, visited, neighbour, directions, f);
        }
    }
}
