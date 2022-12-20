use chumsky::prelude::*;
use day_13::{parser, Expr};

fn main() {
    let input = include_str!("../../../input/day_13.txt");
    let p = parser();

    let parsed = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| p.parse(line))
        .collect::<Result<Vec<Expr>, Vec<Simple<char>>>>();

    let mut packets = match parsed {
        Ok(ok) => ok,
        Err(err) => return err.iter().for_each(|err| eprintln!("{}", err)),
    };

    // Push the divider packets.
    // [[2]] eq 2 and [[6]] eq 6, as for the ordering we have, v == [v]
    packets.push(Expr::Num(2));
    packets.push(Expr::Num(6));

    packets.sort_unstable();

    let first_divider = packets.iter().position(|e| e == &Expr::Num(2)).unwrap() + 1;
    let second_divider = packets.iter().rposition(|e| e == &Expr::Num(6)).unwrap() + 1;

    let decoder_key = first_divider * second_divider;

    println!(
        "indexOf divider [[2]] = {}, indexOf divider [[6]] = {}, decoder key = {}",
        first_divider, second_divider, decoder_key
    );
}
