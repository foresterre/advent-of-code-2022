use day_07::FileSystem;

fn main() {
    let input = include_str!("../../../input/day_07.txt");
    let fs = FileSystem::from_str(input);

    println!("{}", fs.filter_size(|&&size| size <= 100000).sum::<u32>());
}
