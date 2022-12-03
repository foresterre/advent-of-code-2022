use day_03::{intersect, priority};

fn main() {
    let input = include_bytes!("../../../input/day_03.txt");
    let lines = input.split(|&byte| byte == b'\n').collect::<Vec<&[u8]>>();

    let sum_of_priorities = lines
        .chunks(3)
        .map(|chunks| intersect(chunks.iter().map(|chunk| chunk.iter().copied())))
        .map(priority) // map to priorities
        .sum::<u32>();

    println!("{}", sum_of_priorities);
}
