use day_03::{intersect, priority};

fn main() {
    let input = include_bytes!("../../../input/day_03.txt");
    let lines = input.split(|&byte| byte == b'\n').collect::<Vec<&[u8]>>();

    let sum_of_priorities = lines
        .chunks(3)
        .map(|chunks| {
            let a = chunks[0];
            let b = chunks[1];
            let c = chunks[2];

            intersect([a.iter().cloned(), b.iter().cloned(), c.iter().cloned()])
        })
        .map(|item| priority(item)) // map to priorities
        .sum::<u32>();

    println!("{}", sum_of_priorities);
}
