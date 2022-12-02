use std::str::FromStr;
use day_02::{Error, Game, Tournament};

fn main() {
    let input = include_str!("../../../input/day_02.txt");

    let games = input.lines()
        .map(parse)
        .collect::<Result<Vec<_>, Error>>()
        .unwrap();

    let tournament = Tournament::new(&games);

    println!("my projected score: {}", tournament.my_score());
}

fn parse(line: &str) -> Result<Game, Error> {
    line.split_once(' ')
        .ok_or_else(|| Error::ParseGame { line: line.to_owned() })
        .and_then(|(lhs, rhs)| Ok(Game::new(
            rhs.parse()?,
            lhs.parse()?,
        )))
}
