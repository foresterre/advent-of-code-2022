fn main() {
    let input = include_str!("../../../input/day_04.txt");

    println!(
        "{}",
        input
            .lines()
            .map(|line| line.splitn(4, |c| c == '-' || c == ','))
            .map(|mut items| (
                items.next().unwrap().parse::<u32>().unwrap()
                    ..=items.next().unwrap().parse::<u32>().unwrap(),
                items.next().unwrap().parse::<u32>().unwrap()
                    ..=items.next().unwrap().parse::<u32>().unwrap(),
            ))
            .filter(|(a, b)| { a.clone().any(|section| b.contains(&section)) })
            .count()
    );
}
