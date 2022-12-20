use chumsky::prelude::*;
use day_13::{parser, Expr};
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../../input/day_13.txt");
    let input = input.split("\n\n");

    let p = parser();

    let parsed = input
        .map(|lines| {
            let (l, r) = lines.split_once('\n').unwrap();

            Ok((p.parse(l)?, p.parse(r)?))
        })
        .collect::<Result<Vec<(Expr, Expr)>, Vec<Simple<char>>>>();

    let packets = match parsed {
        Ok(ok) => ok,
        Err(err) => return err.iter().for_each(|err| eprintln!("{}", err)),
    };

    let answer = packets
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l.cmp(r) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("{}", answer);
}
