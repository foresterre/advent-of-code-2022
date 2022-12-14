use std::path::Path;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../input/day_08.txt");

    let input = include_str!("../input/day_08.txt");
    let (matrix, width, height) = generate_matrix(input);

    let template = format!(
        "pub const MATRIX: vectrix::Matrix<u8, {width}, {height}> = vectrix::matrix![{}];\n\
         pub const WIDTH: usize = {width};\n\
         pub const HEIGHT: usize = {height};\n",
        matrix,
        width = width,
        height = height,
    );

    let out = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out).join("matrix.rs");

    fs::write(&path, &template).unwrap();
}

fn generate_matrix(input: &str) -> (String, usize, usize) {
    let mut height = 0_usize;
    let width = input.lines().next().unwrap().chars().count();

    let matrix = input
        .lines()
        .map(|line| {
            height += 1;

            let mut buffer = String::with_capacity(width);
            let mut line = line.chars().peekable();

            while let Some(c) = line.next() {
                buffer.push(c);
                if line.peek().is_some() {
                    buffer.push(',')
                }
            }

            buffer
        })
        .collect::<Vec<String>>()
        .join(";");

    (matrix, width, height)
}
