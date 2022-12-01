fn main() {
    let input = include_str!("../../../input/day_01.txt");

    let max = input
        .split("\n\n")
        .map(|elf| {
            elf.split_ascii_whitespace()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();

    println!("{}", max);
}
