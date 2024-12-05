use advent_of_code::FUNCS;

fn main() {
    let day = FUNCS.len();
    let (f1, f2) = FUNCS[day - 1];
    let input = std::fs::read_to_string(format!("inputs/{}.txt", day)).unwrap();
    println!("Part 1 - {}", f1(&input));
    println!("Part 2 - {}", f2(&input));
}
