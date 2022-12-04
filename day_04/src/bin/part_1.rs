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
            .filter(|(a, b)| {
                a.contains(b.start()) && a.contains(b.end())
                    || b.contains(a.start()) && b.contains(a.end())
            })
            .count()
    );
}
