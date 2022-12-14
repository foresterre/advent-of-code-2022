mod input {
    include!(concat!(env!("OUT_DIR"), "/matrix.rs"));
}

use std::io::BufRead;

fn main() {
    let input: vectrix::Matrix<u8, 99, 99> = input::MATRIX;
    let (w, h) = (input::WIDTH, input::HEIGHT);

    let output = input
        .iter_columns()
        .map(|col| col[w - 1])
        .collect::<Vec<_>>();

    println!("{}", input);
    println!("{:?}", output);
}
