use day_07::FileSystem;

fn main() {
    let input = include_str!("../../../input/day_07.txt");
    let fs = FileSystem::from_str(input);

    let free = fs.free_space();

    println!(
        "{}",
        fs.filter_size(|&&size| size >= 30000000 - free)
            .min()
            .unwrap()
    );
}
