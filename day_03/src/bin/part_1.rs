use day_03::{intersect, priority};

fn main() {
    let input = include_bytes!("../../../input/day_03.txt");

    let sum_of_priorities = input
        .split(|&byte| byte == b'\n')
        .map(|line| line.split_at(line.len() / 2)) // split into equally sized compartments
        .map(|(l, r)| intersect([l.iter().cloned(), r.iter().cloned()])) // find the intersection of the compartments
        .map(|item| priority(item)) // map to priorities
        .sum::<u32>();

    println!("{}", sum_of_priorities);
}
