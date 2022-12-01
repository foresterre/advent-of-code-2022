use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../../input/day_01.txt");

    let top_3 = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<BTreeSet<usize>>()
        .into_iter()
        .rev()
        .take(3)
        .sum::<usize>();

    println!("{}", top_3);
}
