const WINDOW: usize = 4;

fn main() {
    let input = include_bytes!("../../../input/day_06.txt");

    println!(
        "{}",
        input
            .windows(WINDOW)
            .enumerate()
            .find_map(|(i, window)| {
                ((window[0] != window[1] && window[0] != window[2] && window[0] != window[3])
                    && (window[1] != window[2] && window[1] != window[3])
                    && (window[2] != window[3]))
                    .then_some(i + WINDOW)
            })
            .unwrap()
    );
}
