const WINDOW: usize = 14;

fn main() {
    let input = include_bytes!("../../../input/day_06.txt");

    println!(
        "{}",
        input
            .windows(WINDOW)
            .enumerate()
            .find_map(|(i, window)| {
                (1..WINDOW)
                    .all(|i| !window[i..].contains(&window[i - 1]))
                    .then_some(i + WINDOW)
            })
            .unwrap()
    );
}
