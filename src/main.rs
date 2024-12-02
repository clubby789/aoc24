use advent_of_code::FUNCS;

fn main() {
    let (f1, f2) = FUNCS.last().unwrap();
    let input = std::fs::read_to_string(format!("inputs/{}.txt", FUNCS.len())).unwrap();
    println!("Part 1 - {}", f1(&input));
    println!("Part 2 - {}", f2(&input));
}
