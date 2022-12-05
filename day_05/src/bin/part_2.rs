use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../../../input/day_05.txt");

    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let mut store = BTreeMap::<usize, Vec<char>>::new();

    stacks.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    store.entry(i).or_default().push(c);
                }
            })
    });

    moves
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace().skip(1).step_by(2);

            let count = iter.next().unwrap().parse::<usize>().unwrap();
            let from = iter.next().unwrap().parse::<usize>().unwrap();
            let to = iter.next().unwrap().parse::<usize>().unwrap();

            (count, from - 1, to - 1)
        })
        .for_each(|(count, from, to)| {
            let items = {
                let stack = store.entry(from).or_default();
                let len = stack.len();
                let boxes = stack[len - count..].to_owned();
                stack.truncate(len - count);

                boxes
            };

            store.entry(to).or_default().extend_from_slice(&items);
        });

    println!(
        "{}",
        store
            .values()
            .map(|stack| stack.last().copied().unwrap_or_default())
            .collect::<String>()
    );
}
